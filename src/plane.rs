
use glm::{angle, cross, distance, dot, make_vec3, TVec3};
use crate::color::RGB;
use crate::primitives::{get_perp_vec, reflect_about_vec};
use crate::sphere::{Object, Ray, RayIntersection};
pub struct Plane{
    normal: TVec3<f32>,
    point: TVec3<f32>
        
}

impl Plane{
    pub fn create_point_normal(point: TVec3<f32>, normal: TVec3<f32>) -> Self{
        return Plane{
            normal: normal,
            point: point
        };
    }

    pub fn create_three_point(p1: TVec3<f32>, p2: TVec3<f32>, p3: TVec3<f32>) -> Self{
        let (p1_v, p2_v, p3_v) = (p1, p2, p3);
        let normal = cross(&(p2_v - p1_v), &(p3_v-p1_v));
        return Self::create_point_normal(p1, normal);
    }
}

impl Object for Plane{
    fn intersection(&self, r: &Ray) -> Option<RayIntersection>{
        let v = r.origin - self.point; 
        let t = -dot(&self.normal, &v)/(dot(&self.normal, &r.direction));
        let err = 1e-5;
        if(t< err){
            return None;
        }
        let incoming_vector = -t*r.direction;
        let p_v = r.origin + t*r.direction;
        let normal_angle = angle(&self.normal, &incoming_vector);
        let (reflection, perp) = reflect_about_vec(&incoming_vector, &self.normal);
        return Some(RayIntersection{
            origin: r.origin.clone(),
            t: t,
            point: p_v,
            normal: self.normal.clone(),
            normal_angle: normal_angle,
            reflection: reflection,
            perp: get_perp_vec(&self.normal),
            distance: distance(&p_v, &r.origin)
        });
    }

    fn color(&self, p: &TVec3<f32>) -> RGB{
        return RGB::black();
    }

    fn le(&self, p: &TVec3<f32>, v: &TVec3<f32>) -> RGB {
        return RGB::black();
    }

}
