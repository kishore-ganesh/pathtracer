use glm::{TVec3, make_vec3};
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
                float_max(a.pMin.x, b.pMin.x),
                float_max(a.pMin.y, b.pMin.y),
                float_max(a.pMin.z, b.pMin.z),
            ])
        };
    }

    pub fn union_point(a: BoundingBox, b: TVec3<f32>) -> BoundingBox {
        return BoundingBox::union(a, BoundingBox::create(b, b));
    }

    pub fn offset(&self, p: TVec3<f32>) -> TVec3<f32> {
        return make_vec3(
            &[
                p.x/(self.pMax.x-self.pMin.x),
                p.y/(self.pMax.y-self.pMin.y),
                p.z/(self.pMax.z-self.pMin.z)
            ]
        )
    }

    pub fn surface_area(&self) -> f32 {
        let d = self.pMax - self.pMin;
        let l = d.x;
        let b = d.y;
        let h = d.z;
        return 2.0 * (l*b + b*h + h*l );
    }

    pub fn intersection(&self, r: &Ray) -> bool {
        let mut tMin = 0.0;
        let mut tMax = 0.0;
        for i in 0..3 {
            let tCandidateMin = (self.pMin[i] - r.origin[i])/r.direction[i];
            let tCandidateMax = (self.pMax[i]-r.origin[i])/r.direction[i];
            let (tCandidateMin, tCandidateMax) = (float_min(tCandidateMin, tCandidateMax), float_max(tCandidateMin, tCandidateMax));
            tMin = float_max(tMin, tCandidateMin);
            tMax = float_min(tMax, tCandidateMax);
            if tMin>tMax {
                return false;
            } 
        }
        if tMax < 0.0 {
            return false;
        } 
        return true;
    }
}

    
    


