use crate::bounding_box::BoundingBox;
use crate::color::RGB;
use crate::primitives::{Object, Ray, RayIntersection, min_intersection};
use crate::triangle::{NormalType, Triangle};
use glm::{TVec3, make_vec3};

#[derive(Clone, Debug)]
pub struct TriangleMesh {
    pub mesh: Vec<Triangle>,
}

impl TriangleMesh {
    //TODO: make this more efficient later
    pub fn create(triangle_points: Vec<[[f32; 3]; 3]>, normals: Vec<[f32; 3]>) -> Self {
        let mut mesh: Vec<Triangle> = vec![];
        for (index, point) in triangle_points.iter().enumerate() {
            mesh.push(Triangle::create(
                [
                    make_vec3(&point[0]),
                    make_vec3(&point[1]),
                    make_vec3(&point[2]),
                ],
                NormalType::FaceNormal(make_vec3(&normals[index])),
            ));
        }
        TriangleMesh { mesh }
    }
    pub fn create_from(v: Vec<Triangle>) -> Self {
        //debug!("Number of triangles: {}", v.len());
        TriangleMesh { mesh: v }
    }
    pub fn from_positions_and_normals(
        positions: Vec<[f32; 3]>,
        indices: Vec<usize>,
        normals: Vec<[f32; 3]>,
    ) -> Self {
        let mut triangles: Vec<Triangle> = Vec::new();
        let positions: Vec<TVec3<f32>> = positions.iter().map(|x| make_vec3(x)).collect();
        let normals: Vec<TVec3<f32>> = normals.iter().map(|x| make_vec3(x)).collect();

        for index in 0..indices.len() {
            if index % 3 == 0 {
                let triangle_points = [
                    positions[indices[index] as usize],
                    positions[indices[index + 1] as usize],
                    positions[indices[index + 2] as usize],
                ];
                let triangle_normals = [
                    normals[indices[index] as usize],
                    normals[indices[index + 1] as usize],
                    normals[indices[index + 2] as usize],
                ];
                triangles.push(Triangle::create(
                    triangle_points,
                    NormalType::VertexNormals(triangle_normals),
                ));
            }
        }
        TriangleMesh { mesh: triangles }
    }

    pub fn from_positions(positions: Vec<[f32; 3]>, indices: Vec<usize>) -> Self {
        let mut triangles: Vec<Triangle> = Vec::new();
        let positions: Vec<TVec3<f32>> = positions.into_iter().map(|x| make_vec3(&x)).collect();
        for index in 0..indices.len() {
            if index % 3 == 0 {
                let triangle_points = [
                    positions[indices[index] as usize],
                    positions[indices[index + 1] as usize],
                    positions[indices[index + 2] as usize],
                ];
                triangles.push(Triangle::create(triangle_points, NormalType::Inferred))
            }
        }
        TriangleMesh { mesh: triangles }
    }
}

impl Object for TriangleMesh {
    fn intersection(&self, r: &Ray) -> Option<RayIntersection> {
        let mut min_intersection_v = None;
        for triangle in self.mesh.iter() {
            (min_intersection_v, _) =
                min_intersection(min_intersection_v, triangle.intersection(r));
        }
        min_intersection_v
    }

    fn color(&self, _: &TVec3<f32>) -> RGB {
        RGB::black()
    }

    fn le(&self, _: &TVec3<f32>, _: &TVec3<f32>) -> RGB {
        RGB::black()
    }

    fn bounds(&self) -> BoundingBox {
        //TODO: incorrect impl
        panic!("Mesh bounding box called");
    }
}
