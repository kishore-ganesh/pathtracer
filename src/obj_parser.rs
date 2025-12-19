use crate::triangle_mesh::TriangleMesh;
use glm::TVec3;
use obj::{Obj, Vertex, load_obj};
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
    if let Ok(parsed_obj) = parsed_obj {
        let positions = parsed_obj.vertices.iter().map(|x| x.position).collect();
        let indices = parsed_obj.indices.iter().map(|x| *x as usize).collect();
        let normals = parsed_obj.vertices.iter().map(|x| x.normal).collect();
    
        TriangleMesh::from_positions_and_normals(
            positions,
            indices,
            normals
        )
    } else {
        // The normals aren't present in the object file
        let input_buf_reader = BufReader::new(File::open(&path).unwrap());
        let parsed_obj: Obj<Position, u16> = load_obj(input_buf_reader).unwrap();
        let positions = parsed_obj.vertices.iter().map(|x| x.position).collect();
        let indices = parsed_obj.indices.iter().map(|x| *x as usize).collect();
        
        TriangleMesh::from_positions(positions, indices)
    }

}
