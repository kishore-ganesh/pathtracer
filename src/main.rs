extern crate nalgebra_glm as glm;
mod sphere;
mod color;
mod lights;
mod materials;
mod sampler;
mod scene;
mod camera;
//mod pathtracer;
use sphere::{Sphere, Ray, Point};
use color::RGB;
use glm::{TMat4, TVec4, make_mat4x4, make_vec4, transpose};
fn main() {
    let x: Sphere = Sphere::create(10, Point::create(2,3,4));
    let r: Ray = Ray{origin: Point::create(1,1,1), direction: Point::create(1,1,1)};
    x.intersection(r);
    let mut v: Vec<Vec<RGB>> = Vec::new();
    let x = 256;
    let y = 240;
    for i in 0..y {
        
        v.push(vec![RGB::create(255.0,0.0,0.0); x]);
    }

    let a = make_vec4(&[1, 1, 1,1]);
    let b = make_mat4x4(&[1,0,0,1,0,1,0,1,0,0,1,1,1,1,0,1]);
    //println!("{:?}", transpose(&a) * b);
    //color::write_ppm(v, "test.ppm".to_string());

    //println!("Hello, world!");
}
