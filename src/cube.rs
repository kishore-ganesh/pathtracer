use crate::primitives::{Point, transform_mesh, rotate_about_x, rotate_about_y, scale, translate};
use crate::sphere::{Object, Ray, RayIntersection};
use crate::triangle_mesh::TriangleMesh;
use crate::color::RGB;
pub struct Cube{
    side: f32, 
    mesh: TriangleMesh,
}


impl Cube{
    pub fn create(location: Point, rx: f32, ry: f32, side: f32, inside: bool) -> Self{
        let cube_triangles: Vec<[[f32; 3]; 3]> = vec![
            //Front
            [
                [-1.0,-1.0,1.0],
                [-1.0,1.0,1.0],
                [1.0,-1.0,1.0]
            ],
            [
                [-1.0,1.0,1.0],
                [1.0,1.0,1.0],
                [1.0,-1.0,1.0],
            ],
            //Left
            [
                [-1.0,-1.0,-1.0],
                [-1.0,1.0, 1.0],
                [-1.0,-1.0,1.0]
            ],
            [
                [-1.0, -1.0,-1.0],
                [-1.0,1.0,-1.0],
                [-1.0,1.0,1.0]
            ],
            //Back
            [
                [-1.0,-1.0,-1.0],
                [-1.0,1.0,-1.0],
                [1.0,-1.0,-1.0]
            ],
            [
                [-1.0,1.0,-1.0],
                [1.0,1.0,-1.0],
                [1.0,-1.0,-1.0],
            ],
            //Right
            //
            [
                [1.0,-1.0,-1.0],
                [1.0,1.0, 1.0],
                [1.0,-1.0,1.0]
            ],
            [
                [1.0, -1.0,-1.0],
                [1.0,1.0,-1.0],
                [1.0,1.0,1.0]
            ],

            //Bottom
            [
                [-1.0,-1.0,1.0],
                [1.0,-1.0,-1.0],
                [1.0,-1.0,1.0]
            ],
            [
                [-1.0,-1.0,1.0],
                [-1.0,-1.0,-1.0],
                [1.0,-1.0,-1.0]
            ],
            //Top
            [
                [-1.0,1.0,1.0],
                [1.0,1.0,-1.0],
                [1.0,1.0,1.0]
            ],
            [
                [-1.0,1.0,1.0],
                [-1.0,1.0,-1.0],
                [1.0,1.0,-1.0]
            ]
        ];

        let dir = match inside {
            true => -1.0,
            false => 1.0
        };

        let cube_normals: Vec<[f32; 3]> = vec![
            [0.0,0.0,dir],
            [0.0,0.0,dir],
            [-dir,0.0,0.0],
            [-dir,0.0,0.0],
            [0.0,0.0,-dir],
            [0.0,0.0,-dir],
            [dir,0.0,0.0],
            [dir,0.0,0.0],
            [0.0,-dir,0.0],
            [0.0,-dir,0.0],
            [0.0,dir,0.0],
            [0.0,dir,0.0],
        ];    

        let unit_cube_mesh: TriangleMesh = TriangleMesh::create(cube_triangles, cube_normals);


        let transformation_matrix =  rotate_about_x(rx) * rotate_about_y(ry) * scale(side, side, side) * translate(location.x, location.y, location.z);
        let mesh = transform_mesh(&transformation_matrix, &unit_cube_mesh);
        return Cube{
            side: side,
            mesh: mesh
        };
    }
}

impl Object for Cube{
    fn intersection(&self, r: &Ray) -> Option<RayIntersection>{
        return self.mesh.intersection(r);
    }
    fn color(&self, p: &Point) -> RGB {
        return self.mesh.color(p);
    }
}
