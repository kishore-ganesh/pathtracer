//
use std::f32::consts::PI;
use glm::{angle, make_vec3, TVec3};
use rand::Rng;
use crate::color::RGB;
use crate::sphere::{Ray, RayIntersection};
use crate::primitives::get_vec_at_angle;
pub trait Material {
    //TODO: check for better interface
    //For now, this will return a spectrum and a ray in the direction
    fn brdf(&self, r: RayIntersection) -> (RGB, Ray);
    fn brdf_eval(&self, r: &RayIntersection, v: &TVec3<f32>) -> RGB;
}
#[derive(Debug, Copy, Clone)]
pub struct DiffuseMaterial {
    fraction: RGB
}

impl DiffuseMaterial{
    pub fn create(f: RGB) -> Self{
        return DiffuseMaterial{fraction: f};
    }
}

impl Material for DiffuseMaterial{
    fn brdf(&self, r: RayIntersection) -> (RGB, Ray){
        //TODO: make this random direction
        
        let mut rand = rand::thread_rng();
        let degree_angle = rand.gen_range(0.0..90.0);
        let rad_angle = (PI/180.0) * degree_angle;
        let direction = get_vec_at_angle(&r.normal, &r.perp, rad_angle);

        return (self.fraction, Ray::create(r.point, direction));
    }
    fn brdf_eval(&self, r: &RayIntersection, v: &TVec3<f32>) -> RGB{
        //TODO: fill in
        return self.fraction;
        //return RGB::black();
    }
}


#[derive(Debug, Copy, Clone)]
pub struct SpecularMaterial {

}

impl SpecularMaterial{
    pub fn create() -> Self{
        return SpecularMaterial{};
    }
}

impl Material for SpecularMaterial{
    fn brdf(&self, r: RayIntersection) -> (RGB, Ray){
        //TODO: extract out the reflection
        let ray = Ray::create(r.point, r.reflection);
        return (RGB::create(255.0,255.0,255.0), ray);
    }
    fn brdf_eval(&self, r: &RayIntersection, v: &TVec3<f32>) -> RGB{
        let ang = angle(&r.normal, &v);
        let err = 1e-5; //TODO: make error more global, new float class?
        if((ang-r.normal_angle).abs() < err){
            return RGB::create(255.0,255.0,255.0);
        }
        else{
            return RGB::black();
        }
    }
}

//later introduce BRDF
