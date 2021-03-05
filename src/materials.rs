//
use glm::{make_vec3, TVec3};
use crate::color::RGB;
use crate::sphere::{Ray, RayIntersection};
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
        return (self.fraction, Ray::create(r.point, make_vec3(&[1.0,1.0,1.0])));
    }
    fn brdf_eval(&self, r: &RayIntersection, v: &TVec3<f32>) -> RGB{
        //TODO: fill in
        return RGB::black();
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
        return RGB::black();
    }
}

//later introduce BRDF
