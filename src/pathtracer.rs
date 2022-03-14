use glm::{angle, make_vec3};
use std::f32::consts::PI;
use std::thread;
use std::thread::JoinHandle;
use rand::Rng;
use crate::camera::Camera;
use crate::color::{RGB, clamp_rgb};
use crate::primitives::Rect;
use crate::scene::Scene;
use crate::sphere::{RayIntersection, Ray, Object};

use crate::materials::Material;

use indicatif::ProgressBar;
//TODO: make rng part of pathtracer. 
#[derive(Clone)]
pub struct PathTracer{
    xres: i32,
    yres: i32,
    n_samples: i32,
    chunk_size: i32,
//    grid: Vec<Vec<RGB>>,
    roulette_threshold: f32,
    camera: Camera,
    scene: Scene
    
}


fn generate_chunk(p: &mut PathTracer, r: Rect, bar: ProgressBar) -> Vec<Vec<RGB>>{
     
    
    let mut rng = rand::thread_rng();
    let mut grid = vec![vec![RGB::black(); p.chunk_size as usize]; p.chunk_size as usize];
    for yindex in 0..p.chunk_size {
        for xindex in 0..p.chunk_size {
            
            //Average it out
            //
            let y = yindex + (r.top.y as i32);
            let x = xindex + (r.bottom.x as i32);
            let mut radiance = RGB::black();
            // println!("x: {}, y: {}", x, y);
            for sample_index in 0..p.n_samples {
                 //sample = sampler.generate_sample();
                // println!("x: {}, y: {}, sample_index: {}", x, y, sample_index);
                 let sample = [x as f32, y as f32];
                 let e1 = rng.gen::<f32>();
                 let e2 = rng.gen::<f32>();
                 let perturbed_sample = [sample[0] + e1, sample[1] + e2];
                //  println!("{:?} {:?}", sample, perturbed_sample);
                 let ray = p.camera.generate_ray(perturbed_sample);
                 radiance += p.li(ray, &mut rng, 2);
                 
                 //have closest intersection 
                 //toss to find whether to stop 
                 //if stop, sample light source and reutrn radiance 
                 //else if not stop, sample BRDF and cast ray. brdf * Li(ray) + Le
            }
            //TODO fix here
            radiance /= p.n_samples as f32;
            grid[yindex as usize][xindex as usize] = radiance;
        }
    }
    bar.inc((p.chunk_size * p.chunk_size * p.n_samples) as u64);
    return grid;
}


impl PathTracer{
    //Should generate RGB grid 
    pub fn create(xres: i32, yres: i32, n_samples: i32, chunk_size: i32, roulette_threshold: f32, scene: Scene, camera: Camera) -> Self{
        //let grid = vec![vec![RGB::black(); xres as usize]; yres as usize];
        return PathTracer{
            xres: xres,
            yres: yres,
            n_samples: n_samples,
            chunk_size: chunk_size,
//            grid: grid,
            roulette_threshold: roulette_threshold,
            camera: camera,
            scene: scene
        };

    }

    pub fn generate(&mut self) -> Vec<Vec<RGB>>{
        //panic!("Generate called");
        let progress_bar = ProgressBar::new( (self.xres * self.yres * self.n_samples) as u64);
        let mut thread_handles: Vec<Vec<Option<JoinHandle<Vec<Vec<RGB>>>>>> = vec![];

        // Cannot use vec! initialization since JoinHandle is not cloneable
        for y in 0..self.yres/self.chunk_size {
            let mut v: Vec<Option<JoinHandle<Vec<Vec<RGB>>>>> = vec![]; 
            for x in 0..self.xres / self.chunk_size {
                v.push(None);
            }

            thread_handles.push(v);
        }
            //vec![vec![None; (self.xres/self.chunk_size) as usize]; (self.yres/self.chunk_size) as usize] 
        let mut grid = vec![vec![RGB::black(); self.xres as usize]; self.yres as usize];
        for y in 0..((self.yres/self.chunk_size) as i32) {
            for x in 0..((self.xres/self.chunk_size) as i32){
                
                //Average it out
                //
                let mut pt = self.clone();
                let progress_bar_new = progress_bar.clone();
                // println!("{:?}", progress_bar_new.length());
                thread_handles[y as usize][x as usize] = Some(thread::spawn(
                    move || {

                        let region = Rect{
                            bottom: make_vec3(&[(x*pt.chunk_size) as f32, (y*pt.chunk_size + pt.chunk_size-1) as f32, 0.0]), 
                            top: make_vec3(&[(x*pt.chunk_size+pt.chunk_size-1) as f32, (y*pt.chunk_size) as f32, 0.0]
                        )};
                        return generate_chunk(&mut pt, region, progress_bar_new);
                    }
                ));
            }
        }

        for ychunk in 0..self.yres/self.chunk_size {
            for xchunk in 0..self.xres / self.chunk_size {
                //println!("ychunk: {}, xchunk: {}", ychunk, xchunk);
                let thread_result = thread_handles[ychunk as usize][xchunk as usize].take().map(JoinHandle::join);
                match thread_result{ 
                    Some(result) => {
                        //let result = handle.join();
                        match result {
                            Ok(grid_section) => {
                                for yindex in 0..self.chunk_size{
                                    for xindex in 0..self.chunk_size{
                                        let y = ychunk*self.chunk_size + yindex;
                                        let x = xchunk*self.chunk_size + xindex;
                                        grid[y as usize][x as usize] = grid_section[yindex as usize][xindex as usize];                
                                    }
                                }
                            },
                    

                            Err(_) => panic!("Thread result unavailable")
                        };
                    },

                    None => {}
                }
            };
                
        }
        

        //For debugging
        grid[(self.yres/2) as usize][(self.xres/2) as usize] = RGB::create(255.0,0.0,0.0);

        return grid;
    } 
    //TODO: Special value for infinite intersection?
    //Mult by angle for first
    fn check_intersection(&mut self, r: &Ray) -> Option<RayIntersection> {
        return self.scene.bvh_root.intersection(r);

    }
    fn li(&mut self, r: Ray, rand: &mut impl Rng, recursion_depth: i32) -> RGB{

        ////println!("Calculating Li");
        let emitted_radiance = RGB::black();
        let mut path_total = RGB::create(255.0,255.0,255.0);
        let mut prev_path_total = RGB::create(255.0,255.0,255.0);
        let mut running_sum = emitted_radiance;
        let mut prev_intersection: Option<RayIntersection> = None;
        let mut prev_min_index: i32 = -1;
        let mut r_c = r.clone();
        let mut n_iterations = 0;
        

        loop {
            ////println!("iterations: {}", n_iterations);
            
             
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
            
            
            match prev_intersection{
                 //Here, check dir to light source & do running_sum += path_total * f(light_point
                 //-> prev_point -> prev_point)
                 //Need an evaluate function for that? -> need prev_theta and next_theta
                 //Till we have material: hack: if diffuse -> easy, if specular, just check if same
                 //dir else 0 
                Some (ray_intersection) => {
                 //Need to check light obstruction here 
                 //println!("Calculating for light");
                 let (light_color, light_vector, light_distance, pdf) = self.scene.light.sample_radiance(ray_intersection.point, ray_intersection.normal);
                 let shadow_ray = Ray::create(ray_intersection.point, light_vector);
                 let shadow_intersection  = self.check_intersection(&shadow_ray);
                 //println!("Light distance is: {}", light_distance);
                 //TODO: if hits emissive object?
                 //println!("Ray Intersection is: {:?}, Shadow intersection: {:?}  Light vector: {}", ray_intersection,shadow_intersection, light_vector);
                 let mut visible = false;
                 match shadow_intersection {
                     Some(s) => {
                         //println!("Shadow intersected: {:?}", s);
                         //println!("Shadow min index: {}, Current min index: {}", shadow_min_index, prev_min_index);
                         //println!("Shadow distance: {}, Current distance: {}", s.distance, light_distance);
                         if s.distance > light_distance {
                            visible = true;
                         }
                     },
                     None => {
                         //println!("No intersection");
                         visible = true;
                                              
                     }
                 }
                 //println!("Min index: {}", shadow_min_index);
                //  visible = true;
                 //println!("Visible: {}", visible);
                 match visible{
                     true => {
                        
                        let brdf = self.scene.bvh_root.brdf_eval_old(&ray_intersection, &light_vector);
                        // println!("running_sum before: {:?}, path_total: {:?}, light_color: {:?} pdf: {}", running_sum, prev_path_total, light_color, pdf);
                        
                        //TODO: should divide by cos theta
                        running_sum +=  prev_path_total * brdf * light_color * (1.0/pdf);
                        
                        // println!("running_sum after: {:?}, path_total: {:?}, light_color: {:?} pdf: {} brdf: {:?}", running_sum, prev_path_total, light_color, pdf, brdf);
                        
                        

                     },
                     false => {}
                 }
                },    
                None => {}
            }

            if n_iterations > 8 {
                //TODO: Bounce or roulette threshold?
                 break;
                let rand_value = rand.gen::<f32>();
                //println!("Rand value: {}, threshold: {}", rand_value, self.roulette_threshold);
                if rand_value <= self.roulette_threshold {
                    //running_sum = (running_sum) / (1.0-self.roulette_threshold);
                    break;
                }
                else{ 
                    path_total = path_total / (1.0 - self.roulette_threshold);
                    //println!("Clamping path");
                    path_total = clamp_rgb(path_total, -255.0,255.0);

                }
            }

            //NOTE: this should be after prev_intersection since we need the previous cached result within BVHNode
            let  min_intersection = self.check_intersection(&r_c);
            match min_intersection {
                Some(ray_intersection) => {
                    //TODO: pass incoming direction 
                    //TODO: return light sampling here. 
                    
                    
                    // println!("Object intersected");
                    //println!("Ray intersection point: {:?}", ray_intersection.point);
                    //Light radiance to point then multiply by cos theta 
                    
                    let view_vector = r_c.origin - ray_intersection.point;
                    // println!("Origin: {}, point: {}, view_vector: {}", r_c.origin, ray_intersection.point, view_vector);
                    if n_iterations==0 {
                        running_sum += self.scene.bvh_root.le(&ray_intersection.point, &view_vector);
                    }
                    //println!("VIEW angle: {}", angle(&ray_intersection.normal, &view_vector) * 180.0 / PI);
                    let (brdf, ray, pdf) = self.scene.bvh_root.brdf(ray_intersection, view_vector);
                    let ray_angle = angle(&ray_intersection.normal, &ray.direction);
                    //println!("BRDF is: {:?}", brdf);
                    //println!("Ray angle: {}", ray_angle);
                    if ray_angle.cos() < 0.0 {
                        //println!("cos is: {}", ray_angle.cos());
                    }
                    //TODO: make it mul
                    prev_path_total  = path_total;
                    path_total = (path_total * brdf * ray_angle.cos())/pdf;
                    //WARNING: for debugging only. Uncomment if you want to return without bouncing 
                    //return path_total;
                    //println!("PDF is: {}", pdf);
                    //println!("Path total: {:?} brdf: {:?} cos: {} pdf: {}", path_total, brdf, ray_angle.cos(), pdf);
                    r_c = ray;
           },
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
//            //println!("Final running sum: {:?}", running_sum);
            return clamp_rgb(running_sum, -255.0,255.0);

    }
}
