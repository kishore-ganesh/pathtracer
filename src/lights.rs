use crate::color::RGB;
use glm::{TVec3, distance, angle, normalize};

pub trait Light {
    fn radiance(&self, point: TVec3<f32>, normal: TVec3<f32>) -> (RGB, TVec3<f32>);  
}
//TODO: check light source interface 
//
#[derive(Copy, Debug, Clone)]
pub struct PointLight {
    location: TVec3<f32>,
    color: RGB,
    intensity: f32,
}
impl PointLight {
    pub fn create(location: TVec3<f32>, color: RGB, intensity: f32) -> Self{
        return PointLight{location: location, color: color, intensity:  intensity};
    }
}

impl Light for PointLight {
    fn radiance(&self, point: TVec3<f32>, normal: TVec3<f32>) -> (RGB, TVec3<f32>) {
        let dist = distance(&self.location, &point);
        let light_vec = -normalize(&(point - self.location));
        let cos_angle = angle(&light_vec, &normal).cos();
        println!("{}, {:?}, {:?}", cos_angle, self.color, self.color * cos_angle);
        return (self.color * cos_angle, light_vec); 
            //* (self.intensity/dist.powi(2));
    }
}

struct InfiniteLight {

}
