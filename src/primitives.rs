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

pub struct Rect {
    pub bottom: TVec3<f32>,
    pub top: TVec3<f32>
}

impl Rect{
    pub fn create(bottom: TVec3<f32>, top: TVec3<f32>) ->Self {
        return Rect{bottom: bottom, top: top};
    }
}
pub fn pointwise_mul_sum(p1: TVec3<f32>, p2: TVec3<f32>) -> f32 {
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

//Transform for TVec3<f32> and For Vector
//TODO: make transform faster
pub fn transform(transform: &TMat4<f32>, p: &TVec3<f32>) -> TVec3<f32>{
    let mut v = vec3_to_vec4(&p);
    v[3] = 1.0;
    //println!("{:?} {:?}", transform, v);
    let transformed = transpose(&v) * transform;
    //TODO: check for divide by zero
    
    if transformed[3] == 0.0{
        panic!("Divide by zero in transform");
    } 
    
    return make_vec3(&[ transformed[0]/transformed[3], transformed[1]/transformed[3], transformed[2]/transformed[3]]);
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
    //println!("{} {} about perp: {} about_parallel: {}", v, about, about_perpendicular, about_parallel);
    
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

