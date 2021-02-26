extern crate nalgebra_glm as glm;
extern crate rand;
mod sphere;
mod color;
mod lights;
mod materials;
mod sampler;
mod scene;
mod camera;
mod primitives;
mod pathtracer;
use glm::{TMat4, TVec4, make_mat4x4, make_vec4, transpose, make_vec3, vec3_to_vec4, mat4_to_mat3};
use sphere::{Sphere, Ray, Object};
use primitives::Point;
use color::RGB;
use pathtracer::PathTracer;
use scene::Scene;
use camera::Camera;
fn main() {
    let center = Point::create(2.0,3.0,4.0);
    let x: Sphere = Sphere::create(5.0, center.clone());
    let r: Ray = Ray{origin: Point::create(1.0,1.0,1.0), direction: make_vec3(&[1.0,1.0,1.0])};
    x.intersection(&r);
    /*let mut v: Vec<Vec<RGB>> = Vec::new();
    let x = 256;
    let y = 240;
    for i in 0..y {
        
        v.push(vec![RGB::create(255.0,0.0,0.0); x]);
    }

    let a = make_vec4(&[1, 1, 1,1]);
    let b = make_mat4x4(&[1,0,0,1,0,1,0,1,0,0,1,1,1,1,0,1]);
    let c = make_vec3(&[1,2,3]);
    let mut d = vec3_to_vec4(&c);
    d[3] = 1;
    println!("{:?}", d);
    let b_3 = mat4_to_mat3(&b);
    println!("{:?} {:?}", b, b_3);
    //println!("{:?}", transpose(&a) * b);
    //color::write_ppm(&v, "test.ppm".to_string());
    //
    */
    //TODO: link up sphere and cam?
    let screen_res = 512.0;
    let raster_res = 512.0;
    let scene = Scene::create(x);
    let camera = Camera::look_at(Point::create(0.0,0.0,0.0), center.clone(), 1.0, 10.0, screen_res, raster_res); 
    let mut pt = PathTracer::create(raster_res as i32, raster_res as i32, 1, 1.0, scene, camera);
    let grid = pt.generate(); 
    color::write_ppm(&grid, "test.ppm".to_string());
    //println!("Hello, world!");
}
