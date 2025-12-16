use crate::bounding_box::BoundingBox;
use crate::color::RGB;
use crate::sphere::{Object, Ray, RayIntersection, min_intersection};
use crate::triangle::{NormalType, Triangle};
use glm::{make_vec3, TVec3};
use log::debug;

#[derive(Clone, Debug)]
pub struct TriangleMesh {
    pub mesh: Vec<Triangle>,
}

impl TriangleMesh {
    //TODO: make this more efficient later
    pub fn create(triangle_points: Vec<[[f32; 3]; 3]>, normals: Vec<[f32; 3]>) -> Self {
        let mut mesh: Vec<Triangle> = vec![];
        for (index, point) in (&triangle_points).iter().enumerate() {
            mesh.push(Triangle::create(
                [
                    make_vec3(&point[0]),
                    make_vec3(&point[1]),
                    make_vec3(&point[2]),
                ],
                NormalType::FaceNormal(make_vec3(&normals[index])),
            ));
        }
        return TriangleMesh { mesh: mesh };
    }
    pub fn create_from(v: Vec<Triangle>) -> Self {
        //debug!("Number of triangles: {}", v.len());
        return TriangleMesh { mesh: v };
    }
}

impl Object for TriangleMesh {
    fn intersection(&self, r: &Ray) -> Option<RayIntersection> {
        let mut min_intersection_v = None;
        for (_, triangle) in (&self.mesh).iter().enumerate() {
            (min_intersection_v, _) = min_intersection(min_intersection_v, triangle.intersection(r));
        }
        min_intersection_v
    }

    fn color(&self, _: &TVec3<f32>) -> RGB {
        return RGB::black();
    }

    fn le(&self, _: &TVec3<f32>, _: &TVec3<f32>) -> RGB {
        return RGB::black();
    }

    fn bounds(&self) -> BoundingBox {
        //TODO: incorrect impl
        panic!("Mesh bounding box called");
    }
}
