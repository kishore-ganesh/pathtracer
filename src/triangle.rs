use crate::primitives::Point;
use crate::sphere::{Object, Ray, RayIntersection};
struct Triangle{
    points: [Point; 3];

}

//TODO: triangle coord system
impl Object for Triangle{
    fn intersection(&self, r: &Ray) -> Option<RayIntersection> {
        let origin = r.origin;
        let direction = r.direction;
        let b_a = points[1].vector() - points[0].vector();
        let c_a = points[2].vector() - points[0].vector();
        let o_a = origin.vector() - points[0].vector();
        let u = dot(&cross(&o_a, &b_a), &c_a);
        let v = dot(&cross(&-direction, &o_a),&c_a);
        let t= dot(&cross(&-direction, &b_a), &o_a);
        let permitted_range = 0.0..=1.0 

        if(permitted_range.contains(&u) && permitted_range.contains(&v) && permitted_range.contains(&(1-u-v))){
            
        }
        return None;

    } 
}
