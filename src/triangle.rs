use crate::primitives::reflect_about_vec;
use crate::sphere::{Object, Ray, RayIntersection};
use crate::color::RGB;
use crate::bounding_box::BoundingBox;
use glm::{normalize, angle, dot, cross, TVec3, distance, length};
#[derive(Debug, Copy, Clone)]
pub enum NormalType{
    FaceNormal(TVec3<f32>),
    VertexNormals([TVec3<f32>; 3])
}
#[derive(Debug, Copy, Clone)]
pub struct Triangle{
    pub points: [TVec3<f32>; 3],
    pub normal_direction: NormalType
}

impl Triangle {
    pub fn create(points: [TVec3<f32>; 3], normal_direction: NormalType) -> Self
    {
        return Triangle{
            points: points,
            normal_direction: normal_direction
        }
    }

}

fn approx(x: f32, l: f32, r: f32) -> f32{
    let l_dist = (x-l).abs();
    let r_dist = (x-r).abs();
    return x;
    /*
    let err = 1e-6;
    //println!("{}", err);

    if(l_dist <= err){
        return l;
    }
    if(r_dist <= err){
        return r;
    }
    return x;*/
}
//TODO: triangle coord system
impl Object for Triangle{

        fn intersection(&self, r: &Ray) -> Option<RayIntersection> {
        let origin = r.origin;
        let direction = r.direction;
        let b_a = self.points[1] - self.points[0];
        let c_a = self.points[2] - self.points[0];
        let o_a = origin - self.points[0];
        let denom = dot(&cross(&-direction, &b_a), &c_a);
        let mut t = dot(&cross(&o_a, &b_a), &c_a)/denom;
        let mut u = dot(&cross(&-direction, &o_a),&c_a)/denom;
        let mut v = dot(&cross(&-direction, &b_a), &o_a)/denom;
        let permitted_range = 0.0..=1.0;
        //println!("Triangle u: {}, v: {} t: {}", u, v, t);
        u = approx(u, 0.0, 1.0);
        v = approx(v, 0.0, 1.0);
        t = approx(t, 0.0, 1.0);
        let eps = 1e-5;
        let w = approx(1.0-u-v, 0.0,1.0);
        if permitted_range.contains(&u) && permitted_range.contains(&v) && permitted_range.contains(&(w)) && t > eps {
            
            //println!("u: {}, v: {}, t: {}", u, v, t);
            let point = (1.0-u-v) * self.points[0] + u*self.points[1] + v * self.points[2];
            let point_a = point - self.points[0];
            let point_b = point - self.points[1];

            let mut normal = normalize(&cross(&point_a, &point_b));
            match self.normal_direction {
                NormalType::FaceNormal(normal_direction) => {
                    if angle(&normal, &normal_direction)!=0.0 {
                        normal = -normal;
                    }

                },
                NormalType::VertexNormals(normals) => {
                    normal = w * normals[0] + u * normals[1] + v * normals[2]
                }
            }
            
            let origin_vector = origin - point;
            let normal_angle = angle(&normal,&origin_vector);
            let (reflection, perp) = reflect_about_vec(&origin_vector, &normal);
            //TODO: check when changing to triangle coordinates
            return Some(RayIntersection{
                origin: r.origin.clone(),
                t: t, 
                point: point,
                normal: normal, 
                perp: perp,
                normal_angle: normal_angle, 
                reflection: reflection,
                distance: distance(&point, &origin)
            });
            //Reflection


            
        }
        return None;

    }

        fn color(&self, p: &TVec3<f32>) -> RGB{
            let p_v = p;
            let a_v = self.points[0];
            let b_v = self.points[1];
            let c_v = self.points[2];
            let total_area = length(&cross(&(c_v-a_v), &(b_v-a_v))).abs()/2.0;
            let u_area = length(&cross(&(p_v-a_v), &(p_v-c_v))).abs()/2.0;
            let v_area = length(&cross(&(p_v-a_v), &(p_v-b_v))).abs()/2.0;
            let u = u_area/total_area;
            let v = v_area/total_area;
            let w = 1.0-u-v;
            //println!("u: {}, v: {}, w: {}", u, v, w);
            let a_color = RGB::create(255.0,0.0,0.0);
            let b_color = RGB::create(0.0,255.0,0.0);
            let c_color = RGB::create(0.0,0.0,255.0);

            return a_color*w + b_color*u + c_color*v;


        }

        fn le(&self, p: &TVec3<f32>, v: &TVec3<f32>) -> RGB {
            return RGB::black();
        }

        fn bounds(&self) -> BoundingBox {
            let result = BoundingBox::union_point(
                BoundingBox::union_point(
                    BoundingBox::create(
                        self.points[0], self.points[0]
                    ), 
                    self.points[1]), 
                self.points[2]);
            // println!("Bounding box for triangle with points: {:?} is {:?}", self.points, result);
            return result;
        }

        

        
}
