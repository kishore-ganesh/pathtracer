use crate::primitives::{Point, reflect_about_vec};
use crate::sphere::{Object, Ray, RayIntersection};
use glm::{normalize, angle, dot, cross, TVec3, distance};
struct Triangle{
    points: [Point; 3],
    normal_direction: TVec3<f32>
}

//TODO: triangle coord system
impl Object for Triangle{
    fn intersection(&self, r: &Ray) -> Option<RayIntersection> {
        let origin = r.origin;
        let direction = r.direction;
        let b_a = self.points[1].vector() - self.points[0].vector();
        let c_a = self.points[2].vector() - self.points[0].vector();
        let o_a = origin.vector() - self.points[0].vector();
        let u = dot(&cross(&o_a, &b_a), &c_a);
        let v = dot(&cross(&-direction, &o_a),&c_a);
        let t= dot(&cross(&-direction, &b_a), &o_a);
        let permitted_range = 0.0..=1.0; 

        if(permitted_range.contains(&u) && permitted_range.contains(&v) && permitted_range.contains(&(1.0-u-v))){
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
}
