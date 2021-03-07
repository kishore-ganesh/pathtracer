
use glm::{angle, cross, distance, dot, TVec3};
use crate::color::RGB;
use crate::primitives::{Point, reflect_about_vec };
use crate::sphere::{Object, Ray, RayIntersection};
pub struct Plane{
    normal: TVec3<f32>,
    point: Point
        
}

impl Plane{
    pub fn create_point_normal(point: Point, normal: TVec3<f32>) -> Self{
        return Plane{
            normal: normal,
            point: point
        };
    }

    pub fn create_three_point(p1: Point, p2: Point, p3: Point) -> Self{
        let (p1_v, p2_v, p3_v) = (p1.vector(), p2.vector(), p3.vector());
        let normal = cross(&(p2_v - p1_v), &(p3_v-p1_v));
        return Self::create_point_normal(p1, normal);
    }
}

impl Object for Plane{
    fn intersection(&self, r: &Ray) -> Option<RayIntersection>{
        let v = r.origin.vector() - self.point.vector(); 
        let t = -dot(&self.normal, &v)/(dot(&self.normal, &r.direction));
        let err = 1e-5;
        if(t< -err){
            return None;
        }
        let incoming_vector = -t*r.direction;
        let p_v = r.origin.vector() + t*r.direction;
        let normal_angle = angle(&self.normal, &incoming_vector);
        let (reflection, perp) = reflect_about_vec(&incoming_vector, &self.normal);
        return Some(RayIntersection{
            t: t,
            point: Point::create_from_vec3(p_v),
            normal: self.normal.clone(),
            normal_angle: normal_angle,
            reflection: reflection,
            perp: perp,
            distance: distance(&p_v, &r.origin.vector())
        });
    }

    fn color(&self, p: &Point) -> RGB{
        return RGB::black();
    }
}
