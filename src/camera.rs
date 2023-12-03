use glm::{TMat4, make_mat4x4, inverse, cross, normalize, TVec3, make_vec3};

use crate::sphere::Ray;
use crate::primitives::{Rect,scale, translate, transform};
use std::f32::consts::PI;
#[derive(Debug, Copy, Clone)]
pub struct Camera {
    camera_to_world: TMat4<f32>,
    raster_to_world: TMat4<f32>,
}

/*
 * Camera Matrices: CameraToWorld, WorldToCamera, CameraToScreen (Projection), ScreenToNDC,
 * NDCToRaster

impl Camera {
    // Look at should take a position and a position to look at

    fn LookAt(){

    }

}*/

//NOTE: glm already has functions, we are reimplementing some for learning purposes
impl Camera {
    pub fn look_at(from: TVec3<f32>, to: TVec3<f32>, f: f32, n: f32, _: f32, raster_res: f32, fov: f32,region: Rect) -> Self{
        let z = normalize(&(to-from));
        let up = make_vec3(&[ 0.0,1.0,0.0 ]);
        //Should we normalize?
        let x =  cross(&z,  &up);
        let n_up = cross(&x, &z);
        let tangent = ((PI/180.0) * (fov/2.0)).tan();
        //println!("{} {} {}", x, n_up, z);
        //println!("{:?} {:?} {:?}", length(&x), length(&up), length(&z));
        let camera_to_world = make_mat4x4(&[x.x, n_up.x, z.x, from.x as f32, 
                                        x.y, n_up.y, z.y, from.y as f32, 
                                        x.z, n_up.z, z.z, from.z as f32, 
                                        0.0, 0.0, 0.0, 1.0]);
        let camera_to_screen = make_mat4x4(&[
                                           1.0,0.0,0.0,0.0,
                                           0.0,1.0,0.0,0.0,
                                           0.0,0.0,f/(f-n),(-f*n)/(f-n),
                                           0.0, 0.0, 1.0, 0.0]) * scale(1.0/tangent,1.0/tangent,1.0);
        let screen_to_camera = inverse(&camera_to_screen);
       
        //let (region_min_x, region_max_x, region_min_y, region_max_y) = (-50.0)
        let screen_to_raster = translate(-region.bottom.x, -region.top.y, 0.0) * scale(1.0/(region.top.x-region.bottom.x), -1.0/(region.top.y-region.bottom.y), 1.0) * scale(raster_res, raster_res, 1.0);
        let raster_to_screen = inverse(&screen_to_raster);
        //NOTE: If we do vec * Mat, then have to multiply matrices LTR, else RTL
        //Camera -> Screen -> NDC -> Raster 
        let raster_to_world = raster_to_screen * screen_to_camera * camera_to_world;

        //println!("Raster to world is: {:?}", raster_to_world);
        //TODO: check nice way to return it correctly
        return Camera{
            camera_to_world: camera_to_world,
            raster_to_world: raster_to_world
        }

        /*
         * CameraToWorld: x.x x.y x.z 0 
         *                up.x up.y up.z 0 
         *                z.x z.y z.z 0 
         *                from.x from.y from.z 1
         *  CameraToScreen:  
         *  ScreenToNDC: 1/s.x_width 0 0 0 
         *              0   1/s.y_width 0 0
         *              0 0 1 0 
         *              s.x_width/2 s.y_width/2 0 1 
         *  NDCToRaster: image.x_width 0 0 0 
         *              0 image.y_width 0 0 
         *              0 0 image.z_width 0 
         *              0 0 0 1 
         *  TODO: handle FOV later, right now assumed as 45 degrees
         * */
        //TODO: check matrix
        //TODO: check screen space. 
    }
    //TODO: make sample a type
    pub fn generate_ray(&self, sample: [f32; 2]) -> Ray{
        //sample gives raster screen position, ray from world(camera_origin) to world(sample_pos)
       let raster_point = make_vec3(&[ sample[0], sample[1], 0.0 ]);
       let transformed_point = transform(&self.raster_to_world, &raster_point);
       let transformed_origin = transform(&self.camera_to_world, &make_vec3(&[ 0.0,0.0,0.0 ]));
       //println!("Transformed origin is: {:?}", transformed_origin);

       let direction = normalize(&(transformed_point - transformed_origin));
                                         
        //TODO: improve performance here

       return Ray{origin: transformed_origin, direction: direction}; 
    } 

    pub fn get_camera_to_world(&self) -> TMat4<f32> {
        self.camera_to_world
    }

}
