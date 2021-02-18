
use color::RGB;

struct PathTracer{
    xres: i32,
    yres: i32,
    n_samples: i32,
    grid: [[RGB; yres]; xres]
}

impl for PathTracer{
    //Should generate RGB grid 
    fn generate(&mut self){
        for x in 0..self.xres {
            for y in 0..self.yres {
                //Average it out
                //
                radiance = RGB{0.0, 0.0, 0.0};
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
    fn li(&mut self, r: Ray) -> RGB{
        min_intersection = None;//compare None, o = o
        for object in scene.objects() {
            intersection = object.intersection(ray);
            //TODO: Add generic object type later 
            //Closest
            match intersection {
                Some(i) => {
                    min_inter
                }
            }
        }


    }
}
