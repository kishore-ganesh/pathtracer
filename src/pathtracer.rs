
use glm::angle;
use crate::camera::Camera;
use crate::color::RGB;
use crate::scene::Scene;
use crate::sphere::{RayIntersection, Ray, Object};
use rand::Rng;
//TODO: make rng part of pathtracer. 
pub struct PathTracer{
    xres: i32,
    yres: i32,
    n_samples: i32,
    grid: Vec<Vec<RGB>>,
    roulette_threshold: f32,
    camera: Camera,
    scene: Scene
    
}

impl PathTracer{
    //Should generate RGB grid 
    pub fn create(xres: i32, yres: i32, n_samples: i32, roulette_threshold: f32, scene: Scene, camera: Camera) -> Self{
        let grid = vec![vec![RGB::black(); xres as usize]; yres as usize];
        return PathTracer{
            xres: xres,
            yres: yres,
            n_samples: n_samples,
            grid: grid,
            roulette_threshold: roulette_threshold,
            camera: camera,
            scene: scene
        };

    }
    pub fn generate(&mut self) -> Vec<Vec<RGB>>{
        let mut rng = rand::thread_rng();
        for x in 0..self.xres {
            for y in 0..self.yres {
                //Average it out
                //
                let mut radiance = RGB::black();
                for sample_index in 0..self.n_samples {
                     //sample = sampler.generate_sample();
                     println!("x: {}, y: {}, sample_index: {}", x, y, sample_index);
                     let sample = [x as f32, y as f32];
                     let ray = self.camera.generate_ray(sample);
                     radiance += self.li(ray, &mut rng, 2);
                     //have closest intersection 
                     //toss to find whether to stop 
                     //if stop, sample light source and reutrn radiance 
                     //else if not stop, sample BRDF and cast ray. brdf * Li(ray) + Le
                }
                //TODO fix here
                radiance /= (self.n_samples as f32);
                self.grid[y as usize][x as usize] = radiance;
            }
        }

        //For debugging
        self.grid[(self.yres/2) as usize][(self.xres/2) as usize] = RGB::create(255.0,0.0,0.0);

        return self.grid.clone();
    } 
    //TODO: Special value for infinite intersection?
    //Mult by angle for first
    fn check_intersection(&self, r: &Ray) -> (Option<RayIntersection>, i32){
        let mut min_intersection: Option<RayIntersection> = None;//compare None, o = o
        let mut min_index: i32 = -1;
        //let min_object: Option<Object> = None;
        for (index, primitive) in (&self.scene.primitives).into_iter().enumerate() {
            //println!("Before ray object intersection test");

            let intersection = primitive.object.intersection(&r);
            //println!("{:?}", intersection);
            //TODO: Add generic object type later 
            //Closest
            match intersection {
                Some(i) => {
                    println!("Intersection found with object {} at: {:?}", index,i.point);
                    match min_intersection{
                        Some(min_i) => {
                            println!("{} {}", i.distance, min_i.distance);
                            if i.distance < min_i.distance {
                                min_intersection = Some(i);
                                min_index = index as i32;
                            }
                        }
                        None => {
                            min_intersection = Some(i);
                            min_index = index as i32;
                        }
                    }
                    //min_intersection = Some(i); //TODO: make this min
                }
                None =>{}
            }
        }

        return (min_intersection, min_index);

    }
    fn li(&mut self, r: Ray, rand: &mut impl Rng, recursion_depth: i32) -> RGB{
        //println!("Calculating Li");
        let emitted_radiance = RGB::black();
        let mut path_total = emitted_radiance;
        let mut running_sum = emitted_radiance;
        let mut prev_intersection: Option<RayIntersection> = None;
        let mut prev_min_index: i32 = -1;
        let mut r_c = r.clone();
        let mut n_iterations = 0;
        while(true){
            //println!("iterations: {}", n_iterations);
            let (min_intersection, min_index) = self.check_intersection(&r_c);
            
            match prev_intersection{
                 //Here, check dir to light source & do running_sum += path_total * f(light_point
                 //-> prev_point -> prev_point)
                 //Need an evaluate function for that? -> need prev_theta and next_theta
                 //Till we have material: hack: if diffuse -> easy, if specular, just check if same
                 //dir else 0 
                Some (ray_intersection) => {
                 //Need to check light obstruction here 
                 let (light_color, light_vector) = self.scene.light.radiance(ray_intersection.point, ray_intersection.normal);
                 let shadow_ray = Ray::create(ray_intersection.point, light_vector);
                 let (shadow_intersection, _) = self.check_intersection(&r_c);
                 match shadow_intersection {
                     Some(_) => {},
                     None => {
                         let brdf = self.scene.primitives[prev_min_index as usize].material.brdf_eval(&ray_intersection, &light_vector);
                         running_sum +=  path_total * light_color * brdf; 
                     }
                 }
                },    
                None => {}
            }

            if n_iterations > 3 {
                let rand_value = rand.gen::<f32>();
                println!("Rand value: {}, threshold: {}", rand_value, self.roulette_threshold);
                if (rand_value <= self.roulette_threshold){
                    running_sum = (running_sum) / (1.0-self.roulette_threshold);
                    break;
                }
            }
            match min_intersection {
                Some(ray_intersection) => {
                    //TODO: pass incoming direction 
                    //TODO: return light sampling here. 
                    

                    println!("Object {} intersected at recursion depth {}", min_index, recursion_depth);
                    println!("Ray intersection point: {:?}", ray_intersection.point);
                    //Light radiance to point then multiply by cos theta 
                    let (light_color,_) = self.scene.light.radiance(ray_intersection.point, ray_intersection.normal);
                    let (brdf, ray) = self.scene.primitives[min_index as usize].material.brdf(ray_intersection);
                    let ray_angle = angle(&ray_intersection.normal, &ray.direction);
                    path_total += brdf * ray_angle.cos();
                    r_c = ray;
                    //let color = RGB::create(0.0,255.0,127.0); 
                    //let mut mult_color = RGB::black();
                    //println!("Reflection: {}", ray_intersection.reflection);
                    //Only for testing barycentric:
                    //return self.scene.objects[min_index as usize].color(&ray_intersection.point);
                    //if(recursion_depth>0){
                        //mult_color = self.li(Ray::create(ray_intersection.point, ray_intersection.reflection), rand, recursion_depth-1)
                    //}
                    //println!("{:?}", mult_color);

                    //if(mult_color.is_black()){
                      //  return color * ray_intersection.normal_angle.cos();
                    //}
                    //else{
                     //   return color * mult_color * ray_intersection.normal_angle.cos();
                    //}
                    //return color * ray_intersection.normal_angle.cos();
                    //return leht_color * color; 
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
            prev_min_index = min_index;

        }

            return running_sum;

    }
}
