//use std::f32;
//TODO: Implement Disp trait
//
//Implement cube
use std::cmp::{Ord, PartialOrd, PartialEq, Eq, Ordering};
use std::ops;
use std::f32::consts::PI;
use glm::{TMat4, TVec3, make_mat4x4, make_vec3,inverse, length2, matrix_comp_mult, comp_add, normalize, angle, dot, distance, vec4_to_vec3};
use crate::color::RGB;
use crate::materials::Material;
use crate::primitives::{get_perp_vec,reflect_about_vec,transform, transform_vec};
use crate::bounding_box::{BoundingBox};
use crate::triangle_mesh::TriangleMesh;
use std::sync::Arc;
pub trait Object: ObjectClone{

    fn intersection(&self, r: &Ray) -> Option<RayIntersection>;
    fn color(&self, p: &TVec3<f32>) -> RGB;
    fn le(&self, p: &TVec3<f32>, v: &TVec3<f32>) -> RGB;
    fn bounds(&self) -> BoundingBox;
    
}

/*
 * The following is a trick to get clone to work on dyn from:
 * https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object/30353928
 * */
pub trait ObjectClone{
    fn clone_object(&self) -> Box<dyn Object + Send>;
}
impl<T> ObjectClone for T
where T: 'static + Object + Clone + Send{
    fn clone_object(&self) -> Box<dyn Object + Send>{
        return Box::new(self.clone());
    }
}

impl Clone for Box<dyn Object + Send>{
    fn clone(&self) -> Box<dyn Object + Send>{
        return self.clone_object();
    }
}
#[derive(Clone,)]
pub struct Primitive{
    pub object: Arc<dyn Object + Send + Sync>,
    pub material: Arc<dyn Material + Send + Sync>
}


impl Primitive{
    pub fn create(o: Arc<dyn Object + Send + Sync>, m: Arc<dyn Material + Send + Sync>) -> Self{
        return Primitive{object: o, material: m};
    }

    pub fn create_from_mesh(o: &TriangleMesh, m: Arc<dyn Material + Send + Sync>) -> Vec<Self> {
        let mut v: Vec<Self> = vec![];
        for t in &o.mesh {
            v.push(Self::create(Arc::new(t.clone()), m.clone()));
        }
        return v;
    }

    pub fn bounds(&self) -> BoundingBox{
        return self.object.bounds();
    }

    pub fn le(&self, p: &TVec3<f32>, v: &TVec3<f32>) -> RGB{
        return self.object.le(p, v);
    }

    pub fn brdf(&self, r: RayIntersection, v: TVec3<f32>) -> (RGB, Ray, f32){
        return self.material.brdf(r, v);
    }
    pub fn brdf_eval(&self, r: &RayIntersection, v: &TVec3<f32>) -> RGB {
        return self.material.brdf_eval(r, v);
    }

    pub fn color(&self, p: &TVec3<f32>) -> RGB{
        return self.object.color(p);
    }
    
    
}

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub center: TVec3<f32>,
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

    pub fn create_empty() -> Self {
        return Ray{origin: make_vec3(&[0.0,0.0,0.0]), direction: make_vec3(&[0.0,0.0,0.0])};
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
    pub distance: f32

}


pub fn min_intersection(min_intersection_v: Option<RayIntersection>, b: Option<RayIntersection>) -> (Option<RayIntersection>, bool) {
    match min_intersection_v {
        None => {
            return (b, true);
            
        },
        Some(i) => {
            match b{
                Some(j) => {
                    //println!("Triangle {} {} distances: {} {}, t's: {} {}", index,min_index,j.distance, i.distance, j.t, i.t);
                    if j.distance < i.distance {
                        return (b, true);
                    
                    }
                }
                None => {},

            }
        }
    };
    return (min_intersection_v, false);
    
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
    pub fn create(radius: f32, center: TVec3<f32>) -> Self{
        let object_to_world = make_mat4x4(&[
                                          1.0, 0.0, 0.0, center.x as f32, 
                                          0.0, 1.0,0.0, center.y as f32, 
                                          0.0,0.0, 1.0, center.z as f32,
                                          0.0, 0.0, 0.0,1.0
        ]);
        //println!("{:?} {:?}", object_to_world, inverse(&object_to_world));
        return Sphere{center: center, r: radius, object_to_world: object_to_world, world_to_object: inverse(&object_to_world)}
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
        if b*b < 4.0*a*c {
            return None;
        }
        else{
            
            //println!("Original origin: {:?}, Ray origin: {:?}, direction: {}", r.origin,t_origin, t_direction);
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

            if t<=0.001 {
                return None;
            }

            let point  = t_origin + t * t_direction;
            //println!("Direction is: {}", t_direction);
            let incoming_vector = -t*t_direction;
            let normal_vec = normalize(&point);
            let normal_angle = angle(&normal_vec, &incoming_vector);
            let (reflection) = reflect_about_vec(&incoming_vector, &normal_vec);
            
            //TODO: handle refleciton case when perp = 0
            //let other_axis = cross(&normal_vec, &incoming_vector);
            /*
             * New coord system: normal, other_axis, cross(normal, other_axis)
             * rotate about other_axis 
             * inverse transform
             * */
            //println!("{} {}", normal_vec, normal_angle * (180.0/PI));
            //println!("{}", angle(&normal_vec, &point));
            //TODO: change normal to world space
            let world_normal_vec = transform_vec(&self.object_to_world, &normal_vec);
            let world_reflection = normalize(&transform_vec(&self.object_to_world, &reflection));
            let world_point = transform(&self.object_to_world, &point);
            //println!("reflection: {}, world: {}", reflection, world_reflection);
            return Some(RayIntersection{
                origin: r.origin.clone(),
                t: t, point: world_point, 
                normal: world_normal_vec,
                perp: get_perp_vec(&world_normal_vec),
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


    fn le(&self, p: &TVec3<f32>, v: &TVec3<f32>) -> RGB {
        return RGB::black();
    }

    fn bounds(&self) -> BoundingBox {
        return BoundingBox::create(
            glm::vec4_to_vec3(
                &(self.object_to_world * glm::make_vec4(&[
                    -self.r,
                    -self.r,
                    -self.r,
                    1.0
                ]))
            ),
            glm::vec4_to_vec3(
                &(self.object_to_world * glm::make_vec4(&[
                    self.r,
                    self.r,
                    self.r,
                    1.0
                ]))
            )
        )
    }

}
   
