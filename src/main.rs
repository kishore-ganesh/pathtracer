extern crate nalgebra_glm as glm;
mod sphere;
mod color;
mod lights;
mod materials;
mod sampler;
mod scene;
mod camera;
mod primitives;
//mod pathtracer;
use sphere::{Sphere, Ray};
use primitives::Point;
use color::RGB;
use glm::{TMat4, TVec4, make_mat4x4, make_vec4, transpose, make_vec3, vec3_to_vec4, mat4_to_mat3};
fn main() {
    let x: Sphere = Sphere::create(10.0, Point::create(2.0,3.0,4.0));
    let r: Ray = Ray{origin: Point::create(1.0,1.0,1.0), direction: make_vec3(&[1.0,1.0,1.0])};
    x.intersection(r);
    let mut v: Vec<Vec<RGB>> = Vec::new();
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
    //color::write_ppm(v, "test.ppm".to_string());

    //println!("Hello, world!");
}
