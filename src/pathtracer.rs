
use crate::color::RGB;
use crate::sphere::{RayIntersection, Ray};
use rand::Rng;
//TODO: make rng part of pathtracer. 
struct PathTracer{
    xres: i32,
    yres: i32,
    n_samples: i32,
    grid: [[RGB; yres]; xres],
    roulette_threshold: f32,
}

impl PathTracer{
    //Should generate RGB grid 
    fn generate(&mut self){
        let mut rng = rand::thread_rng();
        for x in 0..self.xres {
            for y in 0..self.yres {
                //Average it out
                //
                radiance = RGB::black();
                for sample_index in 0..self.n_samples {
                     sample = sampler.generate_sample();
                     ray = camera.generate_ray(sample);
                     radiance += self.li(ray);
                     //have closest intersection 
                     //toss to find whether to stop 
                     //if stop, sample light source and reutrn radiance 
                     //else if not stop, sample BRDF and cast ray. brdf * Li(ray) + Le
                }

                radiance /= self.n_samples;
                grid[x][y] = radiance;
            }
        }
    } 
    //TODO: Special value for infinite intersection?
    //Mult by angle for first
    fn li(&mut self, r: Ray, rand: &Rng) -> RGB{
        min_intersection = None;//compare None, o = o
        min_object = None;
        for object in scene.objects() {
            intersection = object.intersection(r);
            //TODO: Add generic object type later 
            //Closest
            match intersection {
                Some(i) => {
                    
                }
            }
        }

        match min_intersection {
            RayIntersection => {
                (f_r, direction, pdf ) = min_object.brdf_sample(); //TODO: pass incoming direction 
                to_terminate = rand.gen::<f32>();
                if to_terminate < roulette_threshold {
                    //TODO: return light sampling here. 
                    return RGB::black();
                }
                else{
                    return (f_r/pdf) * Li(object.generate_ray(direction));
                }

                return RGB::black(); 
            },
            None =>  RGB::black()
        }
    }
}
