
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
                     let sample = [x as f32, y as f32];
                     let ray = self.camera.generate_ray(sample);
                     radiance += self.li(ray, &rng, 1);
                     //have closest intersection 
                     //toss to find whether to stop 
                     //if stop, sample light source and reutrn radiance 
                     //else if not stop, sample BRDF and cast ray. brdf * Li(ray) + Le
                }

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
    fn li(&mut self, r: Ray, rand: &impl Rng, recursion_depth: i32) -> RGB{
        //println!("Calculating Li");
        let mut min_intersection: Option<RayIntersection> = None;//compare None, o = o
        let mut min_index = 0;
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
                            if i.distance < min_i.distance {
                                min_intersection = Some(i);
                                min_index = index;
                            }
                        }
                        None => {min_intersection = Some(i);}
                    }
                    //min_intersection = Some(i); //TODO: make this min
                }
                None =>{}
            }
        }

        match min_intersection {
            Some(ray_intersection) => {
                /*(f_r, direction, pdf ) = min_object.brdf_sample(); //TODO: pass incoming direction 
                to_terminate = rand.gen::<f32>();
                if to_terminate < roulette_threshold {
                    //TODO: return light sampling here. 
                    return RGB::black();
                }
                else{
                    return (f_r/pdf) * Li(object.generate_ray(direction));
                }*/

                println!("Object {} intersected at recursion depth {}", min_index, recursion_depth);
                println!("Ray intersection point: {:?}", ray_intersection.point);
                //Light radiance to point then multiply by cos theta 
                let light_color = self.scene.light.radiance(ray_intersection.point, ray_intersection.normal);
                let (brdf, ray) = self.scene.primitives[min_index as usize].material.brdf(ray_intersection);
                match ray{
                    Some(r) => {
                        //TODO: make this general
                        if(recursion_depth <=0){
                            return light_color;
                        }
                        else{                
                            return brdf * ray_intersection.normal_angle.cos() * self.li(r, rand, recursion_depth-1);  
                        }

                    },
                    None => return brdf * ray_intersection.normal_angle.cos()
                }
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
                //return light_color * color; 
            },
            None =>  RGB::black()
        }
    }
}
