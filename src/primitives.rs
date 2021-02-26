use std::fmt::Display;
use std::ops;
use glm::{transpose, mat4_to_mat3, make_mat4x4, make_vec3, vec3_to_vec4, TVec3, TMat4};
/*impl Display for TMat4<f32> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        for i in self{
            write!(f, )
        }
    }
}
*/
#[derive(Copy, Clone, Debug)]
//Should be vector representation?
//TODO: refactor out Point
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,

}

impl Point {
    pub fn create_from_vec3(v: TVec3<f32>) -> Self{
        return Point{x: v[0], y: v[1], z: v[2]};
    }
    pub fn create(x: f32, y: f32, z: f32) -> Self {
        return Point{x: x, y:y, z:z};
    }
    pub fn vector(&self) -> TVec3<f32> {
        return make_vec3(&[self.x as f32, self.y as f32, self.z as f32]);
    }
    pub fn square_sum(&self) -> f32 {
        let (x, y, z) = (self.x, self.y, self.z);
        return x*x + y*y + z*z;
    }
   
}
impl ops::Mul<f32> for Point {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self{
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
pub fn pointwise_mul_sum(p1: Point, p2: Point) -> f32 {
     return p1.x * p2.x + p1.y * p2.y + p1.z * p2.z;
}
pub fn scale(scalex: f32, scaley: f32, scalez: f32) -> TMat4<f32> {
    return make_mat4x4(&[scalex, 0.0,0.0,0.0,
                       0.0,scaley,0.0,0.0,
                       0.0,0.0,scalez,0.0,
                       0.0,0.0,0.0,1.0]);
}

pub fn translate(tx: f32, ty: f32, tz: f32) -> TMat4<f32> {
    return make_mat4x4(&[
                       1.0,0.0,0.0,tx,
                       0.0,1.0,0.0,ty,
                       0.0,0.0,1.0,tz,
                       0.0,0.0,0.0,1.0
    ])
}


//Transform for Point and For Vector
//TODO: make transform faster
pub fn transform(transform: &TMat4<f32>, p: &Point) -> Point{
    let mut v = vec3_to_vec4(&p.vector());
    v[3] = 1.0;
    //println!("{:?} {:?}", transform, v);
    let transformed = transpose(&v) * transform;
    //TODO: check for divide by zero
    
    if transformed[3] == 0.0{
        panic!("Divide by zero in transform");
    } 
    
    return Point::create(transformed[0]/transformed[3], transformed[1]/transformed[3], transformed[2]/transformed[3]);
}

pub fn transform_vec(transform: &TMat4<f32>, v: &TVec3<f32>) -> TVec3<f32> {
    let transform3 = mat4_to_mat3(&transform);
    let transformed = transpose(&v) * transform3;
    let transformed_vec = transpose(&transformed);
    return transformed_vec;
}


