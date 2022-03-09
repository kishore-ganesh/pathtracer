use glm::{make_vec3, TVec3, cross, normalize};
use obj::{load_obj, Obj, Position};
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
    let parsed_obj: Obj<Position, u16> = load_obj(input_buf_reader).unwrap();
    let points: Vec<TVec3<f32>> =  parsed_obj.vertices.clone().into_iter().map(|x| make_vec3(&x.position)).collect();
    let indices: Vec<u16> = parsed_obj.indices;
    let mut point_normals: Vec<(TVec3<f32>, f32)> = points.clone().into_iter().map(|x| (make_vec3(&[0.0,0.0,0.0]), 0.0)).collect();
    

    let mut triangles: Vec<Triangle> = Vec::new();
    for index in 0..indices.len() {
        if index%3==0 {
            let triangle_points = [
                points[indices[index] as usize],
                points[indices[index+1] as usize],
                points[indices[index+2] as usize]
        ];
            //TODO: refactor this asap
            let new_normal = normalize(&cross(&(triangle_points[1]-triangle_points[0]), &(triangle_points[2]-triangle_points[0])));
            point_normals[indices[index] as usize] = ((point_normals[indices[index] as usize].0 + new_normal)/(point_normals[indices[index] as usize].1+1.0), (point_normals[indices[index] as usize].1+1.0));
            point_normals[indices[index+1] as usize] = ((point_normals[indices[index+1] as usize].0 + new_normal)/(point_normals[indices[index+1] as usize].1+1.0), (point_normals[indices[index+1] as usize].1+1.0));
            point_normals[indices[index+2] as usize] = ((point_normals[indices[index+2] as usize].0 + new_normal)/(point_normals[indices[index+2] as usize].1+1.0), (point_normals[indices[index+2] as usize].1+1.0));
            
        }
    }

    for index in 0..indices.len() {
        if(index%3==0){
            let triangle_points = [
                points[indices[index] as usize],
                points[indices[index+1] as usize],
                points[indices[index+2] as usize]
            ];

            let triangle_normals = [
                    normalize(&point_normals[indices[index] as usize].0),
                    normalize(&point_normals[indices[index+1] as usize].0),
                    normalize(&point_normals[indices[index+2] as usize].0)
            ];
            triangles.push(Triangle::create(triangle_points, NormalType::VertexNormals(triangle_normals)));
        }
        
    }
    
    //println!("Normals are: {:?}", normals);
    return TriangleMesh::create_from(triangles);
    
}
