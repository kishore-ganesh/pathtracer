use glm::{make_vec3, TVec3};
use obj::{load_obj, Obj, Vertex};
use crate::triangle::{NormalType, Triangle};
use crate::triangle_mesh::TriangleMesh;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;


pub fn parse<P: AsRef<Path>>(o: TVec3<f32>, path: P) -> TriangleMesh{
    //Read all together
    //v: (x, y, z)
    //vt: (u, v)
    //vn: normal 
    //f: vertex_index/texture_index/normal_index
    //Skip lines starting with #
    let input_buf_reader = BufReader::new(File::open(&path).unwrap());
    let parsed_obj: Obj<Vertex, u16> = load_obj(input_buf_reader).unwrap();
    let points: Vec<TVec3<f32>> =  parsed_obj.vertices.clone().into_iter().map(|x| make_vec3(&x.position)).collect();
    let normals: Vec<TVec3<f32>> =  parsed_obj.vertices.clone().into_iter().map(|x| make_vec3(&x.normal)).collect();
    let indices: Vec<u16> = parsed_obj.indices;

    

    let mut triangles: Vec<Triangle> = Vec::new();
    for index in 0..indices.len() {
        if index%3==0 {
            let triangle_points = [
                points[indices[index] as usize],
                points[indices[index+1] as usize],
                points[indices[index+2] as usize]
            ];
            let triangle_normals = [
                normals[indices[index] as usize],
                normals[indices[index+1] as usize],
                normals[indices[index+2] as usize]
            ];
            triangles.push(Triangle::create(triangle_points, NormalType::VertexNormals(triangle_normals)));
        }
    }
    
    //println!("Normals are: {:?}", normals);
    return TriangleMesh::create_from(triangles);
    
}
