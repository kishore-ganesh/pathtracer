use crate::bounding_volume_hierarchy::BVHIntersectionResult;
use crate::camera::Camera;
use crate::color::{clamp_rgb, RGB};
use crate::scene::Scene;
use crate::primitives::{Primitive, Ray, RayIntersection};
use glm::{angle, TVec3};
use rand::Rng;
use indicatif::ProgressBar;
use std::thread;
//TODO: make rng part of pathtracer.
#[derive(Clone)]
pub struct PathTracer<'a> {
    xres: i32,
    yres: i32,
    n_samples: i32,
    chunk_size: i32,
    roulette_threshold: f32,
    camera: Camera,
    scene: Scene<'a>,
}

fn generate_chunk(p: &mut PathTracer, chunk_start_idx: usize, buf: &mut [RGB], bar: ProgressBar) {
    let mut rng = rand::rng();
    for idx in chunk_start_idx..chunk_start_idx + buf.len() {
        let y = idx / (p.xres as usize);
        let x = idx % (p.xres as usize);
        let mut radiance = RGB::black();
        // debug!("x: {}, y: {}", x, y);
        for _ in 0..p.n_samples {
            //sample = sampler.generate_sample();
            // debug!("x: {}, y: {}, sample_index: {}", x, y, sample_index);
            let sample = [x as f32, y as f32];
            let e1 = rng.random::<f32>();
            let e2 = rng.random::<f32>();
            let perturbed_sample = [sample[0] + e1, sample[1] + e2];
            //  debug!("{:?} {:?}", sample, perturbed_sample);
            let ray = p.camera.generate_ray(perturbed_sample);
            radiance += p.li(ray, &mut rng, 2);

            //have closest intersection
            //toss to find whether to stop
            //if stop, sample light source and reutrn radiance
            //else if not stop, sample BRDF and cast ray. brdf * Li(ray) + Le
        }
        //TODO fix here
        radiance /= p.n_samples as f32;
        buf[idx - chunk_start_idx] = radiance;
    }
    bar.inc((p.chunk_size * p.n_samples) as u64);
}

fn importance_sample_weight(pdf_a: f32, pdf_b: f32) -> f32 {
    pdf_a.powi(2) / (pdf_a.powi(2) + pdf_b.powi(2))
}

#[derive(Debug)]
struct SamplingInfo {
    color: RGB,
    light_pdf: f32,
    brdf_pdf: f32,
}

impl PathTracer<'_> {
    //Should generate RGB grid
    pub fn create<'a>(
        xres: i32,
        yres: i32,
        n_samples: i32,
        chunk_size: i32,
        roulette_threshold: f32,
        scene: Scene<'a>,
        camera: Camera,
    ) -> PathTracer<'a> {
        PathTracer {
            xres,
            yres,
            n_samples,
            chunk_size,
            roulette_threshold,
            camera,
            scene,
        }
    }

    pub fn generate(&mut self) -> Vec<RGB> {
        let progress_bar = ProgressBar::new((self.xres * self.yres * self.n_samples) as u64);
        let mut buf = vec![RGB::black(); (self.xres * self.yres) as usize];
        log::warn!(
            "Launching {} threads",
            ((self.xres * self.yres) / self.chunk_size)
        );

        thread::scope(|s| {
            buf.chunks_mut(self.chunk_size as _)
                .enumerate()
                .for_each(|(chunk_idx, chunk)| {
                    let mut pt = self.clone();
                    let progress_bar_new = progress_bar.clone();
                    let chunk_size = pt.chunk_size;
                    s.spawn(move || {
                        generate_chunk(
                            &mut pt,
                            chunk_idx * chunk_size as usize,
                            chunk,
                            progress_bar_new,
                        );
                    });
                });
        });

        //For debugging
        let mid_idx = ((self.xres / 2) * self.yres + self.yres / 2) as usize;
        buf[mid_idx] = RGB::create(255.0, 0.0, 0.0);
        buf
    }
    //TODO: Special value for infinite intersection?
    //Mult by angle for first
    fn check_intersection(&self, r: &Ray) -> Option<BVHIntersectionResult> {
        self.scene.bvh_root.intersection(r)
    }

    fn is_point_visible_from_light(
        &self,
        point: TVec3<f32>,
        light_vector: TVec3<f32>,
        light_distance: f32,
    ) -> bool {
        let shadow_ray = Ray::create(point, light_vector);
        let shadow_intersection = self.check_intersection(&shadow_ray);
        match shadow_intersection {
            Some(r) => r.intersection.distance > light_distance,
            None => true,
        }
    }

    fn sample_light(
        &self,
        ray_intersection: &RayIntersection,
        primitive: &Primitive,
        prev_path_total: RGB,
    ) -> Option<SamplingInfo> {
        let (light_color, light_vector, light_distance, light_pdf) = self
            .scene
            .light
            .sample_radiance(ray_intersection.point, ray_intersection.normal);
        let visible =
            self.is_point_visible_from_light(ray_intersection.point, light_vector, light_distance);
        if visible {
            let (brdf, brdf_pdf) = primitive.brdf_eval(ray_intersection, &light_vector);
            Some(SamplingInfo {
                color: prev_path_total * brdf * light_color * (1.0 / light_pdf), // TODO: extract this out into a function
                light_pdf,
                brdf_pdf,
            })
        } else {
            None
        }
    }

    fn sample_brdf(
        &self,
        ray_intersection: &RayIntersection,
        primitive: &Primitive,
        prev_path_total: RGB,
    ) -> Option<SamplingInfo> {
        let view_vector = ray_intersection.origin - ray_intersection.point;
        let (brdf, ray, brdf_pdf) = primitive.brdf(*ray_intersection, view_vector);
        let radiance_info = self
            .scene
            .light
            .radiance_info(&ray, ray_intersection.normal)?;
        let visible = self.is_point_visible_from_light(
            ray_intersection.point,
            radiance_info.light_vector,
            radiance_info.light_distance,
        );
        if visible {
            Some(SamplingInfo {
                color: prev_path_total * brdf * radiance_info.light_color * (1.0 / brdf_pdf),
                light_pdf: radiance_info.light_pdf,
                brdf_pdf,
            })
        } else {
            None
        }
    }
    fn li(&mut self, r: Ray, rand: &mut impl Rng, _: i32) -> RGB {
        ////debug!("Calculating Li");
        let emitted_radiance = RGB::black();
        let mut path_total = RGB::create(255.0, 255.0, 255.0);
        let mut prev_path_total = RGB::create(255.0, 255.0, 255.0);
        let mut running_sum = emitted_radiance;
        let mut prev_intersection: Option<BVHIntersectionResult> = None;
        let mut current_ray = r;
        let mut n_iterations = 0;

        loop {
            ////debug!("iterations: {}", n_iterations);

            //Uncomment for debugging BRDF:
            /*match min_intersection {
                Some(r) => {
                     let (light_color, light_vector, light_distance,  _) = self.scene.light.sample_radiance(r.point, r.normal);
                let brdf = self.scene.primitives[min_index as usize].material.brdf_eval(&r, &light_vector);
                return brdf * light_color
                        //ray_intersection.normal_angle.cos()
                },

                None => {
                    return RGB::black();
                }
            }*/

            match prev_intersection {
                //Here, check dir to light source & do running_sum += path_total * f(light_point
                //-> prev_point -> prev_point)
                //Need an evaluate function for that? -> need prev_theta and next_theta
                //Till we have material: hack: if diffuse -> easy, if specular, just check if same
                //dir else 0
                Some(bvh_intersection) => {
                    //Need to check light obstruction here
                    //debug!("Calculating for light");

                    if self
                        .scene
                        .bvh_root
                        .get_primitive(bvh_intersection.primitive_idx)
                        .material
                        .is_delta()
                    {
                        unimplemented!()
                        // Get vector from BRDF
                    } else if self.scene.light.is_delta() {
                        unimplemented!()
                        // Get vector from light source
                    } else {
                        // Importance sample
                        let primitive = self
                            .scene
                            .bvh_root
                            .get_primitive(bvh_intersection.primitive_idx);
                        let light_res = self.sample_light(
                            &bvh_intersection.intersection,
                            primitive,
                            prev_path_total,
                        );
                        let brdf_res = self.sample_brdf(
                            &bvh_intersection.intersection,
                            primitive,
                            prev_path_total,
                        );

                        if light_res.is_some() && brdf_res.is_some() {
                            let light_res = light_res.unwrap();
                            let brdf_res = brdf_res.unwrap();
                            running_sum += light_res.color
                                * importance_sample_weight(light_res.light_pdf, light_res.brdf_pdf);
                            running_sum += brdf_res.color
                                * importance_sample_weight(brdf_res.brdf_pdf, brdf_res.light_pdf)
                        } else {
                            // TODO: confirm if you do importance samplign with weight
                            running_sum += light_res
                                .map(|light_res| {
                                    light_res.color
                                        * importance_sample_weight(
                                            light_res.light_pdf,
                                            light_res.brdf_pdf,
                                        )
                                })
                                .unwrap_or_default();
                            running_sum += brdf_res
                                .map(|brdf_res| {
                                    brdf_res.color
                                        * importance_sample_weight(
                                            brdf_res.brdf_pdf,
                                            brdf_res.light_pdf,
                                        )
                                })
                                .unwrap_or_default();
                        }

                        // 1. light
                    }
                }
                None => {}
            }

            if n_iterations > 8 {
                //TODO: Bounce or roulette threshold?
                break;
                let rand_value = rand.random::<f32>();
                //debug!("Rand value: {}, threshold: {}", rand_value, self.roulette_threshold);
                if rand_value <= self.roulette_threshold {
                    //running_sum = (running_sum) / (1.0-self.roulette_threshold);
                    break;
                } else {
                    path_total /= 1.0 - self.roulette_threshold;
                    //debug!("Clamping path");
                    path_total = clamp_rgb(path_total, -255.0, 510.0);
                }
            }

            //NOTE: this should be after prev_intersection since we need the previous cached result within BVHNode
            let min_intersection = self.check_intersection(&current_ray);
            match min_intersection {
                Some(bvh_intersection) => {
                    //TODO: pass incoming direction
                    //TODO: return light sampling here.

                    // debug!("Object intersected");
                    //debug!("Ray intersection point: {:?}", ray_intersection.point);
                    //Light radiance to point then multiply by cos theta

                    let view_vector = current_ray.origin - bvh_intersection.intersection.point;
                    // debug!("Origin: {}, point: {}, view_vector: {}", current_ray.origin, ray_intersection.point, view_vector);
                    let primitive = self
                        .scene
                        .bvh_root
                        .get_primitive(bvh_intersection.primitive_idx);
                    if n_iterations == 0 {
                        running_sum +=
                            primitive.le(&bvh_intersection.intersection.point, &view_vector);
                    }

                    let (brdf, ray, pdf) =
                        primitive.brdf(bvh_intersection.intersection, view_vector);
                    let ray_angle = angle(&bvh_intersection.intersection.normal, &ray.direction);
                    //debug!("BRDF is: {:?}", brdf);
                    //debug!("Ray angle: {}", ray_angle);
                    if ray_angle.cos() < 0.0 {
                        //debug!("cos is: {}", ray_angle.cos());
                    }
                    //TODO: make it mul
                    prev_path_total = path_total;
                    path_total = (path_total * brdf * ray_angle.cos()) / pdf;
                    //WARNING: for debugging only. Uncomment if you want to return without bouncing
                    //return path_total;
                    //debug!("PDF is: {}", pdf);
                    //debug!("Path total: {:?} brdf: {:?} cos: {} pdf: {}", path_total, brdf, ray_angle.cos(), pdf);
                    current_ray = ray;
                }
                None => {
                    //TODO: check this, maybe we can sample another direction?
                    break;
                    //return running_sum;
                    /*match recursion_depth{
                      -1 => RGB::create(255.0,255.0,255.0),
                      _ => RGB::black()

                    }*/
                }
            }
            n_iterations += 1;
            prev_intersection = min_intersection;
            //prev_min_index = min_index
        }
        //            //debug!("Final running sum: {:?}", running_sum);
        clamp_rgb(running_sum, -255.0, 255.0)
    }
}
