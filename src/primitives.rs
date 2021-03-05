use std::fmt::Display;
use std::ops;
use glm::{transpose, mat4_to_mat3, make_mat4x4, make_vec3, vec3_to_vec4, TVec3, TMat4, dot, is_null, normalize};
use crate::{Triangle, TriangleMesh};
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

pub fn rotate_about_x(angle: f32) -> TMat4<f32> {
    return make_mat4x4(&[
                       1.0,0.0,0.0,0.0,
                       0.0,angle.cos(), angle.sin(), 0.0,
                       0.0,-angle.sin(), angle.cos(), 0.0,
                       0.0,0.0,0.0,1.0
    ]);
}
pub fn rotate_about_y(angle: f32) -> TMat4<f32>{
    return make_mat4x4(&[
                       angle.cos(), 0.0, angle.sin(), 0.0,
                       0.0, 1.0, 0.0, 0.0,
                       -angle.sin(), 0.0, angle.cos(), 0.0,
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

pub fn transform_triangle(m: &TMat4<f32>, t: &Triangle) -> Triangle{
    let mut points = t.points.clone();
    for (index, point) in (&t.points).iter().enumerate(){
        points[index] = transform(m, point);
    }
    //iter().map(|x| transform(M, x)).collect();
    return Triangle::create(points, transform_vec(m, &t.normal_direction));
}
pub fn transform_mesh(transform: &TMat4<f32>, m: &TriangleMesh) -> TriangleMesh{
    //TODO: make this better
    let mut mesh = m.mesh.clone();
    for (index,triangle) in (&m.mesh).iter().enumerate(){
        mesh[index] = transform_triangle(transform, triangle);
    }

    return TriangleMesh::create_from(mesh);
}

pub fn reflect_about_vec(v: &TVec3<f32>, about: &TVec3<f32>) -> (TVec3<f32>, TVec3<f32>){
    //NOTE: this assumes both rooted in same point 
    let normalized_about = normalize(&about);
    let about_parallel = dot(&normalized_about, &v) * normalized_about;
    let about_perpendicular = v - about_parallel;
    println!("{} {} about perp: {} about_parallel: {}", v, about, about_perpendicular, about_parallel);
    
    if(is_null(&about_perpendicular, 0.0)){
        return (-about_parallel, normalize(&about_perpendicular));
    }
    else{
        return (about_parallel - about_perpendicular, normalize(&about_perpendicular));
    }
}

/*pub fn get_perp_vec(n: &TVec3<f32>) -> TVec3<f32>{
    if(n.x==0.0 && n.y == 0.0 && n.z==0.0){
        panic!("All zero in perp");
    }

    let first_nz = n.position(|x| x !=0.0);
    let (second_nz, third_nz) = match first_nz {
        0 => (1,2), 
        1 => (2,0),
        2 => (0,1)
    }
    let mut perp_n = [0.0,0.0,0.0];
    perp_n[second_nz] = 0.0;
    perp_n[first_nz] = v[third_nz];
    perp_n[third_nz] = v[first_nz];
    return make_vec3(&perp_n);

    
}*/

pub fn get_vec_at_angle(n: &TVec3<f32>, h: &TVec3<f32>,angle: f32) -> TVec3<f32>{

    //TODO: look at left/right
    //Make coord system with n being the y axis, rotate by angle, get it back to our coordinat
    //esystemi 
    //
    return n * angle.cos() + h * angle.sin();
    
}

