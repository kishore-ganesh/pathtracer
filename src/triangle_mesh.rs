use glm::{make_vec3, TVec3};
use crate::sphere::{Object, Ray, RayIntersection};
use crate::triangle::Triangle;
use crate::color::RGB;
pub struct TriangleMesh{
    pub mesh: Vec<Triangle>
}

impl TriangleMesh{
    //TODO: make this more efficient later
    pub fn create(triangle_points: Vec<[[f32; 3]; 3]>, normals: Vec<[f32;3]>) -> Self{
        let mut mesh: Vec<Triangle> = vec![];
        for (index, point) in (&triangle_points).iter().enumerate(){
            mesh.push(Triangle::create([
                make_vec3(&point[0]),
                make_vec3(&point[1]),
                make_vec3(&point[2])
            ], make_vec3(&normals[index])));
        }
        return TriangleMesh{mesh: mesh}
    }
    pub fn create_from(v: Vec<Triangle>) -> Self{
        return TriangleMesh{mesh: v};
    }
}

impl Object for TriangleMesh{
    fn intersection(&self, r: &Ray) -> Option<RayIntersection> {
        let mut min_intersection = None;
        let mut min_index:i32 = -1;
        
        for (index,triangle) in (&self.mesh).iter().enumerate(){
            //TODO: handle duplication of code
            //println!("Triangle index: {}", index);
            match min_intersection{
                None => {
                    min_intersection = triangle.intersection(r);
                    min_index = (index as i32);
                },
                Some(i) => {
                    match triangle.intersection(r){
                        Some(j) => {
                            //println!("Triangle {} {} distances: {} {}, t's: {} {}", index,min_index,j.distance, i.distance, j.t, i.t);
                            if (j.distance < i.distance){
                                min_intersection = Some(j);
                                min_index = (index as i32);
                            }
                        }
                        None => {},

                    }
                }
            }
        }

        match min_intersection{
            None => {},
            _ =>  {} //println!("Triangle {} intersected", min_index)
            
        }
        return min_intersection;
    }

    fn color(&self, p: &TVec3<f32>) -> RGB{
        return RGB::black();
    }
}
