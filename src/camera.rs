use glm::{TMat4, make_mat4x4, inverse, cross, normalize, transpose, vec3_to_vec4, mat4_to_mat3, TVec3};

use crate::sphere::Ray;
use crate::primitives::{Point, scale, translate, transform, transform_vec};
pub struct Camera {
    from: Point,
    camera_to_world: TMat4<f32>,
    world_to_camera: TMat4<f32>,
    raster_to_screen: TMat4<f32>,
    screen_to_raster: TMat4<f32>,
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
    pub fn look_at(from: Point, to: Point, f: f32, n: f32, screen_res: f32, raster_res: f32) -> Self{
        let z = normalize(&(to.vector()-from.vector()));
        let up = Point::create(0.0,1.0,0.0).vector();
        let x = normalize(&cross(&z,  &up));
        let n_up = normalize(&cross(&x, &z));
        let camera_to_world = make_mat4x4(&[x.x, n_up.x, z.x, from.x as f32, 
                                        x.y, n_up.y, z.y, from.y as f32, 
                                        x.z, n_up.z, z.z, from.z as f32, 
                                        0.0, 0.0, 0.0, 1.0]);
        let world_to_camera = inverse(&camera_to_world);
        let camera_to_screen = make_mat4x4(&[
                                           1.0,0.0,0.0,0.0,
                                           0.0,1.0,0.0,0.0,
                                           0.0,0.0,f/(f-n),(-f*n)/(f-n),
                                           0.0, 0.0, 1.0, 0.0]);
        let screen_to_camera = inverse(&camera_to_screen);
        //TODO: make this res a parameter
 //       let screen_res = 512.0;
   //     let raster_res = 512.0;
        let screen_to_raster = translate(screen_res/2.0, screen_res/2.0, 0.0) * scale(1.0/screen_res, 1.0/screen_res, 1.0) * scale(raster_res, raster_res, 1.0);
        let raster_to_screen = inverse(&screen_to_raster);
        //NOTE: If we do vec * Mat, then have to multiply matrices LTR, else RTL
        //Camera -> Screen -> NDC -> Raster 
        let raster_to_world = raster_to_screen * screen_to_camera * camera_to_world;

        println!("Raster to world is: {:?}", raster_to_world);
        //TODO: check nice way to return it correctly
        return Camera{
            from: from,
            camera_to_world: camera_to_world,
            world_to_camera: world_to_camera,
            raster_to_screen: raster_to_screen,
            screen_to_raster: screen_to_raster,
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
       let raster_point = Point::create(sample[0], sample[1], 0.0);
       let transformed_point = transform(&self.raster_to_world, &raster_point);
       let transformed_origin = transform(&self.camera_to_world, &Point::create(0.0,0.0,0.0));
       let direction = normalize(&(transformed_point.vector() - transformed_origin.vector()));
                                         
        //TODO: improve performance here

       return Ray{origin: transformed_origin, direction: direction}; 
    } 

}
