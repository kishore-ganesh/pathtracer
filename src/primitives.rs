use std::fmt::Display;
use std::ops;
use glm::{transpose, mat4_to_mat3, make_mat4x4, make_vec3, vec3_to_vec4, TVec3, TMat4, dot, is_null, normalize};

//TODO: rename to geometric primitives
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

    pub fn create_from_arr(a: [f32; 3]) -> Self{
        return Point{x: a[0], y: a[1], z: a[2]};
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


pub struct Rect {
    pub bottom: Point,
    pub top: Point
}

impl Rect{
    pub fn create(bottom: Point, top: Point) ->Self {
        return Rect{bottom: bottom, top: top};
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

pub fn reflect_about_vec(v: &TVec3<f32>, about: &TVec3<f32>) -> TVec3<f32>{
    //NOTE: this assumes both rooted in same point 
    let normalized_about = normalize(&about);
    let about_parallel = dot(&normalized_about, &v) * normalized_about;
    let about_perpendicular = v - about_parallel;
    println!("{} {} about perp: {} about_parallel: {}", v, about, about_perpendicular, about_parallel);
    if(is_null(&about_perpendicular, 0.0)){
        return -about_parallel;
    }
    else{
        return about_parallel - about_perpendicular;
    }
}


