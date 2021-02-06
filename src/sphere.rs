//use std::f32;
//TODO: Implement Disp trait
use std::ops;
#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point {
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

}

pub struct Ray {
     pub origin: Point,
     pub direction: Point,
}
// derive debug?
impl Sphere {
    pub fn intersection(&self, r: Ray) -> bool {
        //This is wrong, fix this
        let a = r.direction.square_sum();
        let b = 2 * pointwise_mul_sum(r.origin, r.direction); 
        let c = r.origin.square_sum() - self.r*self.r;
        if(b*b < 4*a*c){
            return false
        }
        else{
            let res: f32 =  ((b*b - 4*a*c) as f32).sqrt();
            let r1: f32 = (-b as f32 + res)/((2*a) as f32);
            let r2: f32 = (-b as f32 - res) /((2*a) as f32); //Find better way to do this
            println!("r1: {}, r2: {}", r1, r2)
            // We now know x, y, z Use it to find theta and phi.
            // z = rcostheta, use to find theta 
            // x = rsinthetacosphi, use to find phi 

        }
        return true;
    }
}
