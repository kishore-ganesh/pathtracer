//use std::f32;
//TODO: Implement Disp trait
//
//Implement cube
use std::ops;
use glm::{TMat4, TVec3, make_mat4x4, make_vec3,inverse};
#[derive(Copy, Clone, Debug)]
//Should be vector representation?
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,

}

impl Point {
    pub fn create(x: i32, y: i32, z: i32) -> Self {
        return Point{x: x, y:y, z:z};
    }
    pub fn vector(&self) -> TVec3<f32> {
        return make_vec3(&[self.x as f32, self.y as f32, self.z as f32]);
    }
    fn square_sum(&self) -> i32 {
        let (x, y, z) = (self.x, self.y, self.z);
        return x*x + y*y + z*z;
    }
   
}
impl ops::Mul<i32> for Point {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self{
        let (x, y, z) = (self.x * rhs, self.y * rhs, self.z * rhs);
        return Self{x, y, z};
        
    }
}

impl ops::Add<Point> for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let (x, y, z) = (self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
        return Self{x, y, z};
    }
}
fn pointwise_mul_sum(p1: Point, p2: Point) -> i32 {
     return p1.x * p2.x + p1.y * p2.y + p1.z * p2.z;
}
pub struct Sphere {
    pub r: i32,
    pub object_to_world: TMat4<f32>,
    pub world_to_object: TMat4<f32>

}

pub struct Ray {
     pub origin: Point,
     pub direction: Point,
}

pub struct RayIntersection {
    pub t: f32
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
    pub fn create(radius: i32, centre: Point) -> Self{
        let object_to_world = make_mat4x4(&[1.0, 0.0, 0.0, 
                                        centre.x as f32, 0.0, 1.0,
                                        0.0, centre.y as f32, 0.0,
                                        0.0, 1.0, centre.z as f32,
                                        0.0, 0.0, 0.0,1.0]);
        //println!("{:?} {:?}", object_to_world, inverse(&object_to_world));
        return Sphere{r: radius, object_to_world: object_to_world, world_to_object: inverse(&object_to_world)}
    }
    pub fn intersection(&self, r: Ray) -> Option<RayIntersection> {
        //This is wrong, fix this
        let a = r.direction.square_sum();
        let b = 2 * pointwise_mul_sum(r.origin, r.direction); 
        let c = r.origin.square_sum() - self.r*self.r;
        //TODO:  improve precision
        if(b*b < 4*a*c){
            return None;
        }
        else{
            let res: f32 =  ((b*b - 4*a*c) as f32).sqrt();
            let r1: f32 = (-b as f32 + res)/((2*a) as f32);
            let r2: f32 = (-b as f32 - res) /((2*a) as f32); //Find better way to do this
            if r1 < 0.0 {
                return None;
            }
            if r2 >= 0.0 {
                return Some(RayIntersection{t:r2}) 
            }
            else {
                return Some(RayIntersection{t: r1})
            }
            //Check which one is closer
            println!("r1: {}, r2: {}", r1, r2)
            // We now know x, y, z Use it to find theta and phi.
            // z = rcostheta, use to find theta 
            // x = rsinthetacosphi, use to find phi 

        }
        return Some(RayIntersection{t: 1.0});
    }
}
