use std::cmp::Ordering;

use crate::{NormalType, Triangle, TriangleMesh, bounding_box::BoundingBox, color::RGB, materials::Material};
use glm::{
    dot, is_null, make_mat4x4, make_vec3, mat4_to_mat3, normalize, transpose, vec3_to_vec4,
    vec4_to_vec3, TMat4, TVec3,
};

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

pub fn transform_triangle(m: &TMat4<f32>, t: &Triangle) -> Triangle {
    let mut points = t.points.clone();
    for (index, point) in (&t.points).iter().enumerate() {
        points[index] = transform(m, point);
    }
    //iter().map(|x| transform(M, x)).collect();
    match t.normal_direction {
        NormalType::FaceNormal(n) => {
            return Triangle::create(points, NormalType::FaceNormal(transform_vec(m, &n)));
        }
        NormalType::VertexNormals(v) => {
            //TODO: correct
            return Triangle::create(points, NormalType::VertexNormals(v));
        }
        NormalType::Inferred => unimplemented!(),
    }
}
pub fn transform_mesh(transform: &TMat4<f32>, m: &TriangleMesh) -> TriangleMesh {
    //TODO: make this better
    let mut mesh = m.mesh.clone();
    for (index, triangle) in (&m.mesh).iter().enumerate() {
        mesh[index] = transform_triangle(transform, triangle);
    }

    return TriangleMesh::create_from(mesh);
}

pub fn reflect_about_vec(v: &TVec3<f32>, about: &TVec3<f32>) -> TVec3<f32> {
    //NOTE: this assumes both rooted in same point
    //v is pointing in same direction of normal
    let normalized_about = normalize(&about);
    let about_parallel = dot(&normalized_about, &v) * normalized_about;

    //debug!("Cosine angle is: {}", 57.29 * (dot(&normalized_about, &normalized_v)).acos());
    return 2.0 * about_parallel - v;
}

pub fn get_perp_vec(n: &TVec3<f32>) -> TVec3<f32> {
    if is_null(&n, 0.0) {
        // debug!("All zero in perp");
        panic!("All zero in perp");
    }
    let mut first_nz = 0;
    if n.x != 0.0 {
        first_nz = 0;
    } else if n.y != 0.0 {
        first_nz = 1;
    } else if n.z != 0.0 {
        first_nz = 2;
    };
    let (second_nz, third_nz) = match first_nz {
        0 => (1, 2),
        1 => (2, 0),
        2 => (0, 1),
        _ => {
            panic!("First nz invalid")
        }
    };
    let mut perp_n = [0.0, 0.0, 0.0];
    perp_n[second_nz] = 0.0;
    perp_n[first_nz] = n[third_nz];
    perp_n[third_nz] = -n[first_nz];
    return make_vec3(&perp_n);
}

pub fn get_vec_at_angle(n: &TVec3<f32>, h: &TVec3<f32>, angle: f32) -> TVec3<f32> {
    //TODO: look at left/right
    //Make coord system with n being the y axis, rotate by angle, get it back to our coordinat
    //esystemi
    //
    return n * angle.cos() + h * angle.sin();
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: TVec3<f32>,
    pub direction: TVec3<f32>,
    pub inv_direction: TVec3<f32>,
}

impl Ray {
    pub fn create(origin: TVec3<f32>, direction: TVec3<f32>) -> Self {
        return Ray {
            origin: origin,
            direction: direction,
            inv_direction: make_vec3(&[1.0, 1.0, 1.0]).component_div(&direction),
        };
    }

    pub fn create_empty() -> Self {
        return Ray {
            origin: make_vec3(&[0.0, 0.0, 0.0]),
            direction: make_vec3(&[0.0, 0.0, 0.0]),
            inv_direction: make_vec3(&[f32::INFINITY, f32::INFINITY, f32::INFINITY]),
        };
    }
}


#[derive(Debug, Clone, Copy)]
pub struct RayIntersection {
    pub t: f32,
    pub origin: TVec3<f32>,
    pub point: TVec3<f32>,
    pub normal: TVec3<f32>,
    pub perp: TVec3<f32>,
    pub normal_angle: f32,
    pub reflection: TVec3<f32>,
    pub distance: f32,
}

impl PartialEq<RayIntersection> for RayIntersection {
    fn eq(&self, other: &RayIntersection) -> bool {
        return self.distance == other.distance;
    }
}

impl PartialOrd<RayIntersection> for RayIntersection {
    fn partial_cmp(&self, other: &RayIntersection) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

pub fn min_intersection<T: PartialOrd>(
    min_intersection_v: Option<T>,
    b: Option<T>,
) -> (Option<T>, bool) {
    match min_intersection_v.as_ref() {
        None => {
            return (b, true);
        }
        Some(i) => match b.as_ref() {
            Some(j) => {
                if j < i {
                    return (b, true);
                }
            }
            None => {}
        },
    };
    return (min_intersection_v, false);
}


pub trait Object: Send + Sync + ObjectClone {
    fn intersection(&self, r: &Ray) -> Option<RayIntersection>;
    fn color(&self, p: &TVec3<f32>) -> RGB;
    fn le(&self, p: &TVec3<f32>, v: &TVec3<f32>) -> RGB;
    fn bounds(&self) -> BoundingBox;
}

/*
 * The following is a trick to get clone to work on dyn from:
 * https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object/30353928
 * */
pub trait ObjectClone {
    fn clone_object(&self) -> Box<dyn Object>;
}
impl<T> ObjectClone for T
where
    T: 'static + Object + Clone,
{
    fn clone_object(&self) -> Box<dyn Object> {
        return Box::new(self.clone());
    }
}

impl Clone for Box<dyn Object> {
    fn clone(&self) -> Box<dyn Object> {
        return self.clone_object();
    }
}

pub struct Primitive {
    pub object: Box<dyn Object>,
    pub material: Box<dyn Material>,
}

impl Primitive {
    pub fn create(o: Box<dyn Object>, m: Box<dyn Material>) -> Self {
        return Primitive {
            object: o,
            material: m,
        };
    }

    pub fn create_from_mesh(o: &TriangleMesh, m: Box<dyn Material>) -> Vec<Self> {
        let mut v: Vec<Self> = vec![];
        for t in &o.mesh {
            v.push(Self::create(Box::new(t.clone()), m.clone()));
        }
        return v;
    }

    pub fn bounds(&self) -> BoundingBox {
        return self.object.bounds();
    }

    pub fn le(&self, p: &TVec3<f32>, v: &TVec3<f32>) -> RGB {
        return self.object.le(p, v);
    }

    pub fn brdf(&self, r: RayIntersection, v: TVec3<f32>) -> (RGB, Ray, f32) {
        return self.material.brdf(r, v);
    }
    pub fn brdf_eval(&self, r: &RayIntersection, v: &TVec3<f32>) -> (RGB, f32) {
        return self.material.brdf_eval(r, v);
    }

    pub fn color(&self, p: &TVec3<f32>) -> RGB {
        return self.object.color(p);
    }
}

