//
use crate::color::RGB;
use crate::sphere::{Ray, RayIntersection};
pub trait Material {
    //TODO: check for better interface
    //For now, this will return a spectrum and a ray in the direction
    fn brdf(&self, r: RayIntersection) -> (RGB, Option<Ray>);
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
    fn brdf(&self, r: RayIntersection) -> (RGB, Option<Ray>){
        return (self.fraction, None);
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
    fn brdf(&self, r: RayIntersection) -> (RGB, Option<Ray>){
        //TODO: extract out the reflection
        let ray = Ray::create(r.point, r.reflection);
        return (RGB::create(255.0,255.0,255.0), Some(ray));
    }
}

//later introduce BRDF
