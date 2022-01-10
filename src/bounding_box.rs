use glm::{TVec3, make_vec3, abs};
use std::cmp::{max, min};
use crate::sphere::{Ray, RayIntersection};

#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
    pMin: TVec3<f32>,
    pMax: TVec3<f32>    
}

fn float_min(a: f32, b: f32) -> f32 {
    if a < b {
        return a;
    }
    return b;

}

fn float_max(a: f32, b: f32) -> f32 {
    if a < b {
        return b;
    }
    return a;

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
    pub fn create(pMin: TVec3<f32>, pMax: TVec3<f32>) -> BoundingBox {
        return BoundingBox{pMin: pMin, pMax: pMax};
    }

    pub fn centroid(&self) -> TVec3<f32> {
        return (self.pMin + self.pMax)/2.0;
    }
    pub fn union(a: BoundingBox, b: BoundingBox) -> BoundingBox {
        return BoundingBox{
            pMin: make_vec3(&[
                float_min(a.pMin.x, b.pMin.x),
                float_min(a.pMin.y, b.pMin.y),
                float_min(a.pMin.z, b.pMin.z),
            ]),
            pMax: make_vec3(&[
                float_max(a.pMax.x, b.pMax.x),
                float_max(a.pMax.y, b.pMax.y),
                float_max(a.pMax.z, b.pMax.z),
            ])
        };
    }

    pub fn union_point(a: BoundingBox, b: TVec3<f32>) -> BoundingBox {
        return BoundingBox::union(a, BoundingBox::create(b, b));
    }

    pub fn offset(&self, p: TVec3<f32>) -> TVec3<f32> {
        assert!(self.pMin.x <= self.pMax.x && self.pMin.y <= self.pMax.y && self.pMin.z <= self.pMax.z);
        let offset_p = p - self.pMin;
        return make_vec3(
            &[
                offset_p.x/(self.pMax.x-self.pMin.x),
                offset_p.y/(self.pMax.y-self.pMin.y),
                offset_p.z/(self.pMax.z-self.pMin.z)
            ]
        )
    }

    pub fn surface_area(&self) -> f32 {
        println!("pMin is: {:?} and pMax is: {:?}", self.pMin, self.pMax);
        if !(self.pMin.x <= self.pMax.x && self.pMin.y <= self.pMax.y && self.pMin.z <= self.pMax.z){
            return 0.0;
        }
        let d = self.pMax - self.pMin;
        let l = d.x;
        let b = d.y;
        let h = d.z;
        return 2.0 * (l*b + b*h + h*l );
    }

    pub fn intersection(&self, r: &Ray) -> bool {
        let mut tMin = 0.0;
        let mut tMax = f32::MAX;
        //println!("Box is: {:?} {:?}, ray is: {:?}", self.pMin, self.pMax, r);
        assert!(self.pMin.x <= self.pMax.x && self.pMin.y <= self.pMax.y && self.pMin.z <= self.pMax.z);
        let mut res = true;
        for i in 0..3 {
            let tCandidateMin = (self.pMin[i] - r.origin[i])/r.direction[i];
            let tCandidateMax = (self.pMax[i]-r.origin[i])/r.direction[i];
            let (tCandidateMin, tCandidateMax) = (float_min(tCandidateMin, tCandidateMax), float_max(tCandidateMin, tCandidateMax));
            if tCandidateMax < 0.0 {
                res = false;
            } 
            tMin = float_max(tMin, tCandidateMin);
            tMax = float_min(tMax, tCandidateMax);
            if tCandidateMin>tCandidateMax {
                 res = false;
            } 
        }
        if tMin>tMax {
            res = false;
        } 
        if tMax < 0.0 {
            res = false;
        } 
        if res {
            //println!("Res is: {}", res);
        }
        
        return res;
    }

    pub fn maximum_extent(&self) -> usize {
        let extents = abs(&(self.pMax - self.pMin));
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

    
    


