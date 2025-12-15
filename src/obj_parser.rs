use crate::triangle::{NormalType, Triangle};
use crate::triangle_mesh::TriangleMesh;
use glm::{make_vec3, TVec3};
use log::debug;
use obj::{load_obj, Obj, Vertex};
use obj_parser_external::{ObjResult, Position};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn parse<P: AsRef<Path>>(_: TVec3<f32>, path: P) -> TriangleMesh {
    //Read all together
    //v: (x, y, z)
    //vt: (u, v)
    //vn: normal
    //f: vertex_index/texture_index/normal_index
    //Skip lines starting with #
    let input_buf_reader = BufReader::new(File::open(&path).unwrap());
    let parsed_obj: ObjResult<Obj<Vertex, u16>> = load_obj(input_buf_reader);
    let mut triangles: Vec<Triangle> = Vec::new();
    if let Ok(parsed_obj) = parsed_obj {
        let points: Vec<TVec3<f32>> = parsed_obj
            .vertices
            .clone()
            .into_iter()
            .map(|x| make_vec3(&x.position))
            .collect();
        let normals: Vec<TVec3<f32>> = parsed_obj
            .vertices
            .clone()
            .into_iter()
            .map(|x| make_vec3(&x.normal))
            .collect();
        let indices: Vec<u16> = parsed_obj.indices;

        for index in 0..indices.len() {
            if index % 3 == 0 {
                let triangle_points = [
                    points[indices[index] as usize],
                    points[indices[index + 1] as usize],
                    points[indices[index + 2] as usize],
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
    } else if let Err(_) = parsed_obj {
        // The normals aren't present in the object file
        let input_buf_reader = BufReader::new(File::open(&path).unwrap());
        let parsed_obj: Obj<Position, u16> = load_obj(input_buf_reader).unwrap();
        let points: Vec<TVec3<f32>> = parsed_obj
            .vertices
            .clone()
            .into_iter()
            .map(|x| make_vec3(&x.position))
            .collect();
        let indices = &parsed_obj.indices;
        for index in 0..indices.len() {
            if index % 3 == 0 {
                let triangle_points = [
                    points[indices[index] as usize],
                    points[indices[index + 1] as usize],
                    points[indices[index + 2] as usize],
                ];
                triangles.push(Triangle::create(triangle_points, NormalType::Inferred))
            }
        }
    }

    //debug!("Normals are: {:?}", normals);
    return TriangleMesh::create_from(triangles);
}
