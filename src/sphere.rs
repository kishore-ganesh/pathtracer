//use std::f32;
//TODO: Implement Disp trait
//
//Implement cube
use std::ops;
use std::f32::consts::PI;
use glm::{TMat4, TVec3, make_mat4x4, make_vec3,inverse, length2, matrix_comp_mult, comp_add, normalize, angle};
use crate::primitives::{Point, pointwise_mul_sum,transform, transform_vec};

pub trait Object{

    fn intersection(&self, r: &Ray) -> Option<RayIntersection>;
}
pub struct Sphere {
    pub r: f32,
    pub object_to_world: TMat4<f32>,
    pub world_to_object: TMat4<f32>

}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
     pub origin: Point,
     pub direction: TVec3<f32>,
}

#[derive(Debug, Clone, Copy)]
pub struct RayIntersection {
    pub t: f32,
    pub point: Point,
    pub normal: TVec3<f32>,
    pub normal_angle: f32

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
    pub fn create(radius: f32, centre: Point) -> Self{
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
        let b = 2.0 * comp_add(&(matrix_comp_mult(&t_origin.vector(), &t_direction))); 
        let c = t_origin.square_sum() - self.r*self.r;
        //println!("{} {}", b*b, 4.0*a*c);
        //TODO:  improve precision
        //println!("{:?}", t_direction);
        if(b*b < 4.0*a*c){
            return None;
        }
        else{
            let res: f32 =  ((b*b - 4.0*a*c) as f32).sqrt();
            let r1: f32 = (-b as f32 + res)/((2.0*a) as f32);
            let r2: f32 = (-b as f32 - res) /((2.0*a) as f32); //Find better way to do this
            let mut t = 0.0;
            //println!("r1: {}, r2: {}", r1, r2);
            if r1 < 0.0 {
                return None;
            }
            if r2 >= 0.0 {
                t = r2;
                //intersection = Some(RayIntersection{t:r2}) 
            }
            else {
                t = r1;
                //intersection = Some(RayIntersection{t: r1})
            }

            let point  = t_origin.vector() + t * t_direction;
            let incoming_vector = -t*t_direction;
            let denom = ((-point.x.powi(2) - point.y.powi(2) + self.r.powi(2))).sqrt();
            let df_dx = (-point.x) / denom;
            let df_dy = (-point.y)/ denom;
            let normal_vec = normalize(&make_vec3(&[-df_dx, -df_dy, 1.0]));
            let normal_angle = angle(&normal_vec, &incoming_vector);
            println!("{} {}", normal_vec, normal_angle * (180.0/PI));
            return Some(RayIntersection{t: t, point: Point::create_from_vec3(point), normal: normal_vec, normal_angle: normal_angle});

        }
            //Check which one is closer
            // We now know x, y, z Use it to find theta and phi.
            // z = rcostheta, use to find theta 
            // x = rsinthetacosphi, use to find phi 
            //return intersection;
        
        return None;
    }

}
   
