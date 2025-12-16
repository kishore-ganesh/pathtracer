//use std::f32;
//TODO: Implement Disp trait
//
//Implement cube
use crate::bounding_box::BoundingBox;
use crate::color::RGB;
use crate::primitives::{Object, Ray, RayIntersection, get_perp_vec, reflect_about_vec, transform, transform_vec};
use glm::{
    angle, distance, dot, inverse, length2, make_mat4x4,
    normalize, TMat4, TVec3,
};

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub center: TVec3<f32>,
    pub r: f32,
    pub object_to_world: TMat4<f32>,
    pub world_to_object: TMat4<f32>,
}


//TODO: implement CMP for rayintersection
// derive debug?
// Sphere should know where it is in world space.
// Sphere axes: by default oriented w.r.t Origin, just translated
/* ObjectToWorld: 1 0 0 0
 *                0 1 0 0
 *                0 0 1 0
 *                cx cy cz 1
 *WorldToObject: Inverse(ObjectToWorld)
 * For rays, we should ObjectToWorld(r.origin), ObjectToWorld(r.direction), but in case of
 * direction, should not translate (meaningless)
 * */
impl Sphere {
    pub fn create(radius: f32, center: TVec3<f32>) -> Self {
        let object_to_world = make_mat4x4(&[
            1.0,
            0.0,
            0.0,
            center.x,
            0.0,
            1.0,
            0.0,
            center.y,
            0.0,
            0.0,
            1.0,
            center.z,
            0.0,
            0.0,
            0.0,
            1.0,
        ]);
        //debug!("{:?} {:?}", object_to_world, inverse(&object_to_world));
        Sphere {
            center,
            r: radius,
            object_to_world,
            world_to_object: inverse(&object_to_world),
        }
    }
}
impl Object for Sphere {
    fn intersection(&self, r: &Ray) -> Option<RayIntersection> {
        let duration = std::time::Instant::now();
        //This is wrong, fix this
        let t_origin = transform(&self.world_to_object, &r.origin);
        let t_direction = transform_vec(&self.world_to_object, &r.direction);
        //debug!("{:?}", r.direction);
        //debug!("{:?} {}", t_origin, t_direction);
        let a = length2(&t_direction);
        let b = 2.0 * dot(&t_origin, &t_direction);
        let c = length2(&t_origin) - self.r * self.r;
        //debug!("b: {} 4ac: {}", b*b, 4.0*a*c);
        //TODO:  improve precision
        //debug!("{:?}", t_direction);

        //Check which one is closer
        // We now know x, y, z Use it to find theta and phi.
        // z = rcostheta, use to find theta
        // x = rsinthetacosphi, use to find phi

        if b * b < 4.0 * a * c {
            None
        } else {
            //debug!("Original origin: {:?}, Ray origin: {:?}, direction: {}", r.origin,t_origin, t_direction);
            let res: f32 = ((b * b - 4.0 * a * c) as f32).sqrt();
            let r1: f32 = (-b as f32 + res) / ((2.0 * a) as f32);
            let r2: f32 = (-b as f32 - res) / ((2.0 * a) as f32); //Find better way to do this

            //debug!("r1: {}, r2: {}", r1, r2);
            if r1 <= 0.0 {
                return None;
            }
            let t = if r2 > 0.0 { r2 } else { r1 };

            if t <= 0.001 {
                return None;
            }

            let point = t_origin + t * t_direction;
            //debug!("Direction is: {}", t_direction);
            let incoming_vector = -t * t_direction;
            let normal_vec = normalize(&point);
            let normal_angle = angle(&normal_vec, &incoming_vector);
            let reflection = reflect_about_vec(&incoming_vector, &normal_vec);

            //TODO: handle refleciton case when perp = 0
            //let other_axis = cross(&normal_vec, &incoming_vector);
            /*
             * New coord system: normal, other_axis, cross(normal, other_axis)
             * rotate about other_axis
             * inverse transform
             * */
            //debug!("{} {}", normal_vec, normal_angle * (180.0/PI));
            //debug!("{}", angle(&normal_vec, &point));
            //TODO: change normal to world space
            let world_normal_vec = transform_vec(&self.object_to_world, &normal_vec);
            let world_reflection = normalize(&transform_vec(&self.object_to_world, &reflection));
            let world_point = transform(&self.object_to_world, &point);
            //debug!("reflection: {}, world: {}", reflection, world_reflection);
            Some(RayIntersection {
                origin: r.origin,
                t,
                point: world_point,
                normal: world_normal_vec,
                perp: get_perp_vec(&world_normal_vec),
                normal_angle,
                reflection: world_reflection,
                distance: distance(&world_point, &t_origin),
            })
        }
    }

    fn color(&self, _: &TVec3<f32>) -> RGB {
        RGB::black()
    }

    fn le(&self, _: &TVec3<f32>, _: &TVec3<f32>) -> RGB {
        RGB::black()
    }

    fn bounds(&self) -> BoundingBox {
        BoundingBox::create(
            glm::vec4_to_vec3(
                &(self.object_to_world * glm::make_vec4(&[-self.r, -self.r, -self.r, 1.0])),
            ),
            glm::vec4_to_vec3(
                &(self.object_to_world * glm::make_vec4(&[self.r, self.r, self.r, 1.0])),
            ),
        )
    }
}


#[cfg(test)]
mod tests {
    use glm::make_vec3;

    use super::*;

    #[test]
    fn simple_intersection_test() {
        let sphere = Sphere::create(10.0, make_vec3(&[10.0,10.0,10.0]));
        let ray_origin = make_vec3(&[50.0,1.0,2.0]);
        let ray = Ray::create(ray_origin, normalize(&(sphere.center - ray_origin)));
        let intersection = sphere.intersection(&ray);
        assert!(intersection.is_some());
    }
}