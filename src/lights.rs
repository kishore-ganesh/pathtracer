use glm::{angle, cross, distance, dot, length, normalize, TVec3};
use std::f32::consts::PI;
use rand::Rng;
use crate::color::RGB;
use crate::primitives::{get_perp_vec};
use crate::sphere::{Object, Ray, RayIntersection, Sphere};
use crate::bounding_box::BoundingBox;
pub trait Light: LightClone {
    fn sample_radiance(&self, point: TVec3<f32>, normal: TVec3<f32>) -> (RGB, TVec3<f32>, f32, f32);
}

/*
 * The following is a trick to get clone to work on dyn from:
 * https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object/30353928
 * */
pub trait LightClone{
    fn clone_light(&self) -> Box<dyn Light + Send>;
}
impl<T> LightClone for T
where T: 'static + Light + Clone + Send{
    fn clone_light(&self) -> Box<dyn Light + Send>{
        return Box::new(self.clone());
    }
}

impl Clone for Box<dyn Light + Send>{
    fn clone(&self) -> Box<dyn Light + Send>{
        return self.clone_light();
    }
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
    fn sample_radiance(&self, point: TVec3<f32>, normal: TVec3<f32>) -> (RGB, TVec3<f32>, f32, f32) {
        let dist = distance(&self.location, &point);
        let light_vec = -normalize(&(point - self.location));
        let cos_angle = angle(&light_vec, &normal).cos();
        //////println!("{}, {:?}, {:?}", cos_angle, self.color, self.color * cos_angle);
        return (self.color * cos_angle * self.intensity, light_vec, dist, 1.0); 
            //* (self.intensity/dist.powi(2));
    }
}

struct InfiniteLight {

}

#[derive(Debug, Copy, Clone)]
pub struct SphericalAreaLight{
    sphere: Sphere,
    color: RGB,
    intensity: f32

}


impl SphericalAreaLight{
    pub fn create(sphere: Sphere, color: RGB, intensity: f32) -> Self{
        return SphericalAreaLight{
            sphere: sphere,
            color: color, 
            intensity: intensity
        };
    } 
}

impl Light for SphericalAreaLight{
    fn sample_radiance(&self, point: TVec3<f32>, point_normal: TVec3<f32>) -> (RGB, TVec3<f32>, f32, f32){
        //println!("Sampling light at: {}", point);
        let dist = distance(&point, &self.sphere.center);
        let sin_theta_max = self.sphere.r / dist;
        let theta_max = sin_theta_max.asin();
        let mut rng = rand::thread_rng();
        let e1 = rng.gen::<f32>() * theta_max;
        let e2 = rng.gen::<f32>() * 2.0 * PI;
        let d_s = dist * e1.cos() - (self.sphere.r.powi(2) - dist.powi(2) * e1.sin().powi(2)).sqrt();

        let cos_alpha = (self.sphere.r.powi(2) + dist.powi(2) - d_s.powi(2))/(2.0 * dist * self.sphere.r);
        ////println!("Cos alpha: {}", cos_alpha * (180.0/PI));
        let alpha = cos_alpha.acos();
        let normal = normalize(&(point - self.sphere.center));
        let tangent = normalize(&get_perp_vec(&normal));
        let bitangent = cross(&normal, &tangent);
        //println!("Theta max: {} alpha: {}", theta_max, alpha);
        //println!("numerator: {}, denom: {}", self.sphere.r, dist);
        ////println!("Length of normal: {}, tangent: {}, bitangent: {}", length(&normal), length(&tangent), length(&bitangent));
        ////println!("Dot of normal, tangent is: {}", dot(&normal, &tangent));
        //TODO: refactor out (same thing in Disney BRDF)
        let intersection_point = (normal * cos_alpha + tangent * alpha.sin() * e2.sin() + bitangent * alpha.sin() * e2.cos()) * self.sphere.r + self.sphere.center;
        //println!("Normal: {}, Tangent: {}, Bitangent: {}, Intersection Point: {}", normal, tangent, bitangent, intersection_point);
        ////println!("Length: {}", length(&(intersection_point)));
        let light_vec = -normalize(&(point - intersection_point));
        let theta_area = angle(&(intersection_point - self.sphere.center), &-light_vec);
        let theta_light = angle(&point_normal, &light_vec);

        //println!("Theta Area: {}, Theta Light: {}", theta_area * (180.0/PI), theta_light * (180.0/PI));
        let point_distance = distance(&intersection_point, &point);
        ////println!("Point distance: {}", point_distance);
        let pdf = 1.0 / ((1.0 - theta_max.cos()) *(2.0 * PI));
        let mut res_color = RGB::black();
        if theta_light.cos() > 0.0 {    
            res_color =  self.color * theta_area.cos() * self.intensity * theta_light.cos();
        }
        //println!("{:?} {:?}", res_color, pdf);
        return (res_color, light_vec, point_distance, pdf);
    }


}

impl Object for SphericalAreaLight{
    fn intersection(&self, r: &Ray) -> Option<RayIntersection>{
        return self.sphere.intersection(r);
    }
    fn color(&self, p: &TVec3<f32>) -> RGB{
        return RGB::black();
    }
    fn le(&self, p: &TVec3<f32>, v: &TVec3<f32>) -> RGB {
        let normal = p - self.sphere.center;
        let theta_area = angle(&normal, &v);
        return self.color * theta_area.cos() * self.intensity;
    }
    
    fn bounds(&self) -> BoundingBox {
        return self.sphere.bounds();
    }

}


