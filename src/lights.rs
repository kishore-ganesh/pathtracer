use crate::primitives::Point;
use crate::color::RGB;
use glm::{TVec3, distance, angle, normalize};

pub trait Light {
    fn radiance(&self, point: Point, normal: TVec3<f32>) -> RGB; 
}
//TODO: check light source interface 
//
#[derive(Copy, Debug, Clone)]
pub struct PointLight {
    location: Point,
    color: RGB
}
impl PointLight {
    pub fn create(location: Point, color: RGB) -> Self{
        return PointLight{location: location, color: color};
    }
}

impl Light for PointLight {
    fn radiance(&self, point: Point, normal: TVec3<f32>) -> RGB {
        let dist = distance(&self.location.vector(), &point.vector());
        let light_vec = normalize(&(point.vector() - self.location.vector()));
        let cos_angle = angle(&light_vec, &normal).cos();
        println!("{}", cos_angle);
        return self.color * cos_angle * (1.0/dist.powi(2));
    }
}

struct InfiniteLight {

}
