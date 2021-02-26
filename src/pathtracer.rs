
use crate::camera::Camera;
use crate::color::RGB;
use crate::scene::Scene;
use crate::sphere::{RayIntersection, Ray, Object};
use rand::Rng;
//TODO: make rng part of pathtracer. 
struct PathTracer{
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
    fn create(xres: i32, yres: i32, n_samples: i32, roulette_threshold: f32, scene: Scene, camera: Camera) -> Self{
        let grid = vec![vec![RGB::black(); yres as usize]; xres as usize];
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
    fn generate(&mut self){
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
                     radiance += self.li(ray, &rng);
                     //have closest intersection 
                     //toss to find whether to stop 
                     //if stop, sample light source and reutrn radiance 
                     //else if not stop, sample BRDF and cast ray. brdf * Li(ray) + Le
                }

                radiance /= (self.n_samples as f32);
                self.grid[x as usize][y as usize] = radiance;
            }
        }
    } 
    //TODO: Special value for infinite intersection?
    //Mult by angle for first
    fn li(&mut self, r: Ray, rand: &impl Rng) -> RGB{
        let mut min_intersection: Option<RayIntersection> = None;//compare None, o = o
        //let min_object: Option<Object> = None;
        for object in &self.scene.objects {
            let intersection = object.intersection(&r);
            
            //TODO: Add generic object type later 
            //Closest
            match intersection {
                Some(i) => {
                    min_intersection = Some(i); //TODO: make this min
                }
                None =>{}
            }
        }

        match min_intersection {
            Some(RayIntersection) => {
                /*(f_r, direction, pdf ) = min_object.brdf_sample(); //TODO: pass incoming direction 
                to_terminate = rand.gen::<f32>();
                if to_terminate < roulette_threshold {
                    //TODO: return light sampling here. 
                    return RGB::black();
                }
                else{
                    return (f_r/pdf) * Li(object.generate_ray(direction));
                }*/

                return RGB::create(0.0,255.0,127.0);
            },
            None =>  RGB::black()
        }
    }
}
