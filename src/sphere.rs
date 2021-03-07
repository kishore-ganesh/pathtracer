//use std::f32;
//TODO: Implement Disp trait
//
//Implement cube
use std::cmp::{Ord, PartialOrd, PartialEq, Eq, Ordering};
use std::ops;
use std::f32::consts::PI;
use glm::{TMat4, TVec3, make_mat4x4, make_vec3,inverse, length2, matrix_comp_mult, comp_add, normalize, angle, dot, distance};
use crate::color::RGB;
use crate::materials::Material;
use crate::primitives::{pointwise_mul_sum,transform, transform_vec, reflect_about_vec};

pub trait Object{

    fn intersection(&self, r: &Ray) -> Option<RayIntersection>;
    fn color(&self, p: &TVec3<f32>) -> RGB;
}


pub struct Primitive{
    pub object: Box<dyn Object>,
    pub material: Box<dyn Material>
}


impl Primitive{
    pub fn create(o: Box<dyn Object>, m: Box<dyn Material>) -> Self{
        return Primitive{object: o, material: m};
    }
}
pub struct Sphere {
    pub r: f32,
    pub object_to_world: TMat4<f32>,
    pub world_to_object: TMat4<f32>,

}



#[derive(Debug, Clone, Copy)]
pub struct Ray {
     pub origin: TVec3<f32>,
     pub direction: TVec3<f32>,
}

impl Ray{
    pub fn create(origin: TVec3<f32>, direction: TVec3<f32>) -> Self{
        return Ray{origin: origin, direction: direction};
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RayIntersection {
    pub t: f32,
    pub point: TVec3<f32>,
    pub normal: TVec3<f32>,
    pub perp: TVec3<f32>,
    pub normal_angle: f32,
    pub reflection: TVec3<f32>,
    pub distance: f32

}




/*impl PartialEq for Option<RayIntersection> {
    fn eq(&self, other: &self)
}(/)*/
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
    pub fn create(radius: f32, centre: TVec3<f32>) -> Self{
        let object_to_world = make_mat4x4(&[
                                          1.0, 0.0, 0.0, centre.x as f32, 
                                          0.0, 1.0,0.0, centre.y as f32, 
                                          0.0,0.0, 1.0, centre.z as f32,
                                          0.0, 0.0, 0.0,1.0
        ]);
        //println!("{:?} {:?}", object_to_world, inverse(&object_to_world));
        return Sphere{r: radius, object_to_world: object_to_world, world_to_object: inverse(&object_to_world)}
    }
}
impl Object for Sphere {
    fn intersection(&self, r: &Ray) -> Option<RayIntersection> {
        //This is wrong, fix this
        let t_origin = transform(&self.world_to_object, &r.origin);
        let t_direction = transform_vec(&self.world_to_object, &r.direction);
        //println!("{:?}", r.direction);
        //println!("{:?} {}", t_origin, t_direction);
        let a = length2(&t_direction);
        let b = 2.0 * comp_add(&(matrix_comp_mult(&t_origin, &t_direction))); 
        let c = length2(&t_origin) - self.r*self.r;
        //println!("b: {} 4ac: {}", b*b, 4.0*a*c);
        //TODO:  improve precision
        //println!("{:?}", t_direction);
        if(b*b < 4.0*a*c){
            return None;
        }
        else{
            
            println!("Original origin: {:?}, Ray origin: {:?}, direction: {}", r.origin,t_origin, t_direction);
            let res: f32 =  ((b*b - 4.0*a*c) as f32).sqrt();
            let r1: f32 = (-b as f32 + res)/((2.0*a) as f32);
            let r2: f32 = (-b as f32 - res) /((2.0*a) as f32); //Find better way to do this
            let mut t = 0.0;
            //println!("r1: {}, r2: {}", r1, r2);
            if r1 <= 0.0 {
                return None;
            }
            if r2 > 0.0 {
                t = r2;
                //intersection = Some(RayIntersection{t:r2}) 
            }
            else {
                t = r1;
                //intersection = Some(RayIntersection{t: r1})
            }

            if(t<=0.001){
                return None;
            }

            let point  = t_origin + t * t_direction;
            //println!("Direction is: {}", t_direction);
            let incoming_vector = -t*t_direction;
            let normal_vec = normalize(&point);
            let normal_angle = angle(&normal_vec, &incoming_vector);
            let (reflection, perp) = reflect_about_vec(&incoming_vector, &normal_vec);
            
            //TODO: handle refleciton case when perp = 0
            //let other_axis = cross(&normal_vec, &incoming_vector);
            /*
             * New coord system: normal, other_axis, cross(normal, other_axis)
             * rotate about other_axis 
             * inverse transform
             * */
            println!("{} {}", normal_vec, normal_angle * (180.0/PI));
            //println!("{}", angle(&normal_vec, &point));
            //TODO: change normal to world space
            let world_normal_vec = transform_vec(&self.object_to_world, &normal_vec);
            let world_reflection = normalize(&transform_vec(&self.object_to_world, &reflection));
            let world_point = transform(&self.object_to_world, &point);
            let world_perp = transform_vec(&self.object_to_world, &perp);
            //println!("reflection: {}, world: {}", reflection, world_reflection);
            return Some(RayIntersection{
                t: t, point: world_point, 
                normal: world_normal_vec,
                perp: world_perp,
                normal_angle: normal_angle, 
                reflection: world_reflection, 
                distance: distance(&world_point, &t_origin)});

        }
            //Check which one is closer
            // We now know x, y, z Use it to find theta and phi.
            // z = rcostheta, use to find theta 
            // x = rsinthetacosphi, use to find phi 
            //return intersection;
        
        return None;
    }

    fn color(&self, p: &TVec3<f32>) -> RGB{
        return RGB::black();
    }

}
   
