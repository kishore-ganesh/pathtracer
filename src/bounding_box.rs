use glm::{TVec3, make_vec3, abs};
use crate::sphere::Ray;

#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
    p_min: TVec3<f32>,
    p_max: TVec3<f32>    
}


impl BoundingBox {
    pub fn create_empty() -> BoundingBox {
        return BoundingBox::create(
            glm::make_vec3(&[
                f32::MAX,
                f32::MAX,
                f32::MAX],
            ),
            glm::make_vec3(&[
                f32::MIN,
                f32::MIN,
                f32::MIN
                ],
            ),

        )
    }
    pub fn create(p_min: TVec3<f32>, p_max: TVec3<f32>) -> BoundingBox {
        return BoundingBox{p_min: p_min, p_max: p_max};
    }

    pub fn centroid(&self) -> TVec3<f32> {
        return (self.p_min + self.p_max)/2.0;
    }
    pub fn union(a: BoundingBox, b: BoundingBox) -> BoundingBox {
        return BoundingBox{
            p_min: make_vec3(&[
                a.p_min.x.min(b.p_min.x),
                a.p_min.y.min(b.p_min.y),
                a.p_min.z.min(b.p_min.z),
            ]),
            p_max: make_vec3(&[
                a.p_max.x.max(b.p_max.x),
                a.p_max.y.max(b.p_max.y),
                a.p_max.z.max(b.p_max.z),
            ])
        };
    }

    pub fn union_point(a: BoundingBox, b: TVec3<f32>) -> BoundingBox {
        return BoundingBox::union(a, BoundingBox::create(b, b));
    }

    pub fn offset(&self, p: TVec3<f32>) -> TVec3<f32> {
        debug_assert!(self.p_min.x <= self.p_max.x && self.p_min.y <= self.p_max.y && self.p_min.z <= self.p_max.z);
        let offset_p = p - self.p_min;
        return make_vec3(
            &[
                offset_p.x/(self.p_max.x-self.p_min.x+1e-10),
                offset_p.y/(self.p_max.y-self.p_min.y+1e-10),
                offset_p.z/(self.p_max.z-self.p_min.z+1e-10)
            ]
        )
    }

    pub fn surface_area(&self) -> f32 {
        //println!("p_min is: {:?} and p_max is: {:?}", self.p_min, self.p_max);
        if !(self.p_min.x <= self.p_max.x && self.p_min.y <= self.p_max.y && self.p_min.z <= self.p_max.z){
            return 0.0;
        }
        let d = self.p_max - self.p_min;
        let l = d.x;
        let b = d.y;
        let h = d.z;
        return 2.0 * (l*b + b*h + h*l );
    }

    pub fn intersection(&self, r: &Ray) -> bool {
        let mut t_min: f32 = 0.0;
        let mut t_max: f32 = f32::MAX;
        
        if !(self.p_min.x <= self.p_max.x && self.p_min.y <= self.p_max.y && self.p_min.z <= self.p_max.z) {
            println!("Box is: {:?} {:?}, ray is: {:?}", self.p_min, self.p_max, r);
        }
        debug_assert!(self.p_min.x <= self.p_max.x && self.p_min.y <= self.p_max.y && self.p_min.z <= self.p_max.z);
        let mut res = true;
        //return res;
        for i in 0..3 {
            let t_candidate_min = (self.p_min[i] - r.origin[i])/r.direction[i];
            let t_candidate_max = (self.p_max[i]-r.origin[i])/r.direction[i];
            if t_candidate_min.is_nan() || t_candidate_max.is_nan() {
                //println!("t_candidate_min is: {}, t_candidate_max is: {}, p_min: {:?}, p_max: {:?}", t_candidate_min, t_candidate_max, self.p_min, self.p_max);
            }
            
            //println!("t_candidate_min: {}", t_candidate_min);
            //println!("t_candidate_max: {}", t_candidate_max);
        let (t_candidate_min, t_candidate_max) = (t_candidate_min.min(t_candidate_max), t_candidate_min.max(t_candidate_max));
            //println!("t_candidate_minAF: {}", t_candidate_min);
            //println!("t_candidate_maxAF: {}", t_candidate_max);
            
            t_min = t_min.max(t_candidate_min);
            t_max = t_max.min(t_candidate_max);
            if t_min>t_max {
                //println!("{}  > {}", t_min, t_max);
                res = false;
            } 
        }
        debug_assert!(!t_min.is_nan() && !t_max.is_nan());
        if t_min>t_max {
            res = false;
        } 
        if t_max < 0.0 {
            res = false;
        } 

        
        if res {
            //println!("Res is: {}", res);
        }
        
        return res;
    }

    pub fn maximum_extent(&self) -> usize {
        let extents = abs(&(self.p_max - self.p_min));
        let mx = extents[2].max(extents[1].max(extents[0]));
        if mx == extents[0] {
            return 0;

        }
        if mx == extents[1] {
            return 1;
        }
        return 2;
    }
}

    
    


