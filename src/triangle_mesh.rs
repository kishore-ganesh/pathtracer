use glm::make_vec3;
use crate::primitives::Point;
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
                Point::create_from_arr(point[0]),
                Point::create_from_arr(point[1]),
                Point::create_from_arr(point[2])
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
        for triangle in &self.mesh{
            //TODO: handle duplication of code
            match min_intersection{
                None => min_intersection = triangle.intersection(r),
                Some(i) => {
                    match triangle.intersection(r){
                        Some(j) => {
                            if (j.distance < i.distance){
                                min_intersection = Some(j);
                            }
                        }
                        None => {},

                    }
                }
            }
        }
        return min_intersection;
    }

    fn color(&self, p: &Point) -> RGB{
        return RGB::black();
    }
}
