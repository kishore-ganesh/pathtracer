use glm::{make_vec3, TVec3};
use crate::triangle::{NormalType, Triangle};
use crate::triangle_mesh::TriangleMesh;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub enum ParseResult{
    Point(TVec3<f32>),
    Normal(TVec3<f32>),
    TriangleInfo([(usize, usize); 3]),
    Unknown
}
pub fn parse_line(l: String) -> ParseResult{
    match &l[0..1]{
        "v" => {
            let mut split: Vec<&str> = l.split(" ").collect();
            
            split = split[1..].to_vec();
            let split_converted: Vec<f32> = split.iter().map(|x| x.parse::<f32>().unwrap()).collect();
            
            //println!("{:?}", split);
            match &l[1..2]{
                " " => {
                    return ParseResult::Point(make_vec3(&split_converted[..])); 
                },
                "t" => {
                    return ParseResult::Unknown;
                },
                "n" => {
                    return ParseResult::Normal(make_vec3(&split_converted[..])); 
                }, 
                _ => panic!("Unknown type")
            }
        }, 
        "f" => {
            let mut split: Vec<&str> = l.split(" ").collect();
            split = split[1..].to_vec();
            let triangle_info: Vec<(usize, usize)> = split.iter().map(|x| { 
                                                             let mut point_info = x.split("/");
                                                             println!("Point info: {:?}", point_info);
                                                             let point_index = point_info.nth(0).unwrap().parse::<usize>().unwrap();
                                                             let normal_index = point_info.nth(1).unwrap().parse::<usize>().unwrap();
                                                            return (point_index, normal_index)
            }).collect();
            //println!("{:?} {:?}", triangle_info, normal_index);
            return ParseResult::TriangleInfo([triangle_info[0], triangle_info[1], triangle_info[2]]);

        
        },
        _ => return ParseResult::Unknown
    }

    return ParseResult::Unknown;


}
pub fn parse<P: AsRef<Path>>(o: TVec3<f32>, path: P) -> TriangleMesh{
    //Read all together
    //v: (x, y, z)
    //vt: (u, v)
    //vn: normal 
    //f: vertex_index/texture_index/normal_index
    //Skip lines starting with #
    let mut file_result = File::open(&path);

    let mut points: Vec<TVec3<f32>> = Vec::new();
    let mut normals: Vec<TVec3<f32>> = Vec::new();
    let mut triangles: Vec<Triangle> = Vec::new();
    match file_result {
        Ok(file) => {
            let lines = BufReader::new(file).lines();
            for line in lines{
                if let Ok(l) = line {
                    //println!("{}", l);
                    let result = parse_line(l);
                    match result{
                        ParseResult::Point(p) => points.push(p),
                        ParseResult::Normal(n) => normals.push(n),
                        ParseResult::TriangleInfo(t) => {
                            let triangle_points = [
                                points[t[0].0 - 1],
                                points[t[1].0 - 1],
                                points[t[2].0 - 1]
                            ];
                            let triangle_normals = [
                                normals[t[0].1 - 1],
                                normals[t[1].1 - 1],
                                normals[t[2].1 - 1]
                            ];

                            //println!("Triangle normals: {:?}", triangle_normals);
                            triangles.push(Triangle::create(triangle_points, NormalType::VertexNormals(triangle_normals)));
                        }
                        ParseResult::Unknown => {}
                    }

                }
            }
            

        },

        Err(e) => {
            panic!("Error in opening file: {}", e)
        }
    }
    //println!("Normals are: {:?}", normals);
    return TriangleMesh::create_from(triangles);
    
}
