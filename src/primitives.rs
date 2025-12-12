use glm::{TMat4, TVec3, dot, is_null, make_mat4x4, make_vec3, mat4_to_mat3, normalize, transpose, vec3_to_vec4, vec4_to_vec3};
use crate::{NormalType, Triangle, TriangleMesh};
use log::debug;

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
    //debug!("{:?} {:?}", transform, v);
    let transformed = transpose(&v) * transform;
    //TODO: check for divide by zero
    return vec4_to_vec3(&transpose(&(transformed / transformed[3])));
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
    match t.normal_direction{
        NormalType::FaceNormal(n) => {
            return Triangle::create(points, NormalType::FaceNormal(transform_vec(m, &n)));
        },
        NormalType::VertexNormals(v) => {
            //TODO: correct
            return Triangle::create(points, NormalType::VertexNormals(v));

        },
        NormalType::Inferred => unimplemented!()
    }
}
pub fn transform_mesh(transform: &TMat4<f32>, m: &TriangleMesh) -> TriangleMesh{
    //TODO: make this better
    let mut mesh = m.mesh.clone();
    for (index,triangle) in (&m.mesh).iter().enumerate(){
        mesh[index] = transform_triangle(transform, triangle);
    }

    return TriangleMesh::create_from(mesh);
}

pub fn reflect_about_vec(v: &TVec3<f32>, about: &TVec3<f32>) -> TVec3<f32> {
    //NOTE: this assumes both rooted in same point 
    //v is pointing in same direction of normal
    //debug!("Reflecting {} about {}", v, about);
    let normalized_about = normalize(&about);
    let about_parallel = dot(&normalized_about, &v) * normalized_about;
    
    //debug!("Cosine angle is: {}", 57.29 * (dot(&normalized_about, &normalized_v)).acos());
    return 2.0 * about_parallel - v;
    
}

pub fn get_perp_vec(n: &TVec3<f32>) -> TVec3<f32>{
    if is_null(&n, 0.0) {
        // debug!("All zero in perp");
        panic!("All zero in perp");
    }
    let mut first_nz = 0;
    if n.x != 0.0 {
        first_nz = 0;
    }
    else if n.y != 0.0 {
        first_nz = 1;
    }
    else if n.z != 0.0 {
        first_nz = 2;
    }

    ;
    let (second_nz, third_nz) = match first_nz {
        0 => (1,2), 
        1 => (2,0),
        2 => (0,1),
        _ => {
            panic!("First nz invalid")
        }
    };
    let mut perp_n = [0.0,0.0,0.0];
    perp_n[second_nz] = 0.0;
    perp_n[first_nz] = n[third_nz];
    perp_n[third_nz] = -n[first_nz];
    return make_vec3(&perp_n);

    
}

pub fn get_vec_at_angle(n: &TVec3<f32>, h: &TVec3<f32>,angle: f32) -> TVec3<f32>{

    //TODO: look at left/right
    //Make coord system with n being the y axis, rotate by angle, get it back to our coordinat
    //esystemi 
    //
    return n * angle.cos() + h * angle.sin();
    
}

