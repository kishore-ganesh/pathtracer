use crate::primitives::{Point, reflect_about_vec};
use crate::sphere::{Object, Ray, RayIntersection};
use crate::color::RGB;
use glm::{normalize, angle, dot, cross, TVec3, distance, length};
#[derive(Debug, Copy, Clone)]
pub struct Triangle{
    pub points: [Point; 3],
    pub normal_direction: TVec3<f32>
}

impl Triangle {
    pub fn create(points: [Point; 3], normal_direction: TVec3<f32>) -> Self
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
    let err = 1e-6;
    //println!("{}", err);

    if(l_dist <= err){
        return l;
    }
    if(r_dist <= err){
        return r;
    }
    return x;
}
//TODO: triangle coord system
impl Object for Triangle{

        fn intersection(&self, r: &Ray) -> Option<RayIntersection> {
        let origin = r.origin;
        let direction = r.direction;
        let b_a = self.points[1].vector() - self.points[0].vector();
        let c_a = self.points[2].vector() - self.points[0].vector();
        let o_a = origin.vector() - self.points[0].vector();
        let denom = dot(&cross(&-direction, &b_a), &c_a);
        let mut t = dot(&cross(&o_a, &b_a), &c_a)/denom;
        let mut u = dot(&cross(&-direction, &o_a),&c_a)/denom;
        let mut v = dot(&cross(&-direction, &b_a), &o_a)/denom;
        let permitted_range = 0.0..=1.0;
        //println!("Triangle u: {}, v: {} t: {}", u, v, t);
        u = approx(u, 0.0, 1.0);
        v = approx(v, 0.0, 1.0);
        t = approx(t, 0.0, 1.0);
        let w = approx(1.0-u-v, 0.0,1.0);
        if(permitted_range.contains(&u) && permitted_range.contains(&v) && permitted_range.contains(&(w))){
            
            println!("u: {}, v: {}, t: {}", u, v, t);
            let point = (1.0-u-v) * self.points[0].vector() + u*self.points[1].vector() + v * self.points[2].vector();
            let point_a = point - self.points[0].vector();
            let point_b = point - self.points[1].vector();
            let mut normal = normalize(&cross(&point_a, &point_b));
            if(angle(&normal, &self.normal_direction)!=0.0){
                normal = -normal;
            }

            let origin_vector = origin.vector() - point;
            let normal_angle = angle(&normal,&origin_vector);
            let reflection = reflect_about_vec(&origin_vector, &normal);
            //TODO: check when changing to triangle coordinates
            return Some(RayIntersection{
                t: t, 
                point: Point::create_from_vec3(point),
                normal: normal, 
                normal_angle: normal_angle, 
                reflection: reflection,
                distance: distance(&point, &origin.vector())
            });
            //Reflection


            
        }
        return None;

    }

        fn color(&self, p: &Point) -> RGB{
            let p_v = p.vector();
            let a_v = self.points[0].vector();
            let b_v = self.points[1].vector();
            let c_v = self.points[2].vector();
            let total_area = length(&cross(&(c_v-a_v), &(b_v-a_v))).abs()/2.0;
            let u_area = length(&cross(&(p_v-a_v), &(p_v-c_v))).abs()/2.0;
            let v_area = length(&cross(&(p_v-a_v), &(p_v-b_v))).abs()/2.0;
            let u = u_area/total_area;
            let v = v_area/total_area;
            let w = 1.0-u-v;
            println!("u: {}, v: {}, w: {}", u, v, w);
            let a_color = RGB::create(255.0,0.0,0.0);
            let b_color = RGB::create(0.0,255.0,0.0);
            let c_color = RGB::create(0.0,0.0,255.0);

            return a_color*w + b_color*u + c_color*v;


        }
        

        
}
