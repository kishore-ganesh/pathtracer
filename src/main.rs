mod sphere;
mod color;
mod lights;
mod materials;
mod sampler;
mod scene;
//mod pathtracer;
use sphere::{Sphere, Ray, Point};
use color::RGB;
fn main() {
    let x: Sphere = Sphere{r: 10};
    let r: Ray = Ray{origin: Point{x:1, y:1, z:1}, direction: Point{x:1, y:1, z:1}};
    x.intersection(r);
    let mut v: Vec<Vec<RGB>> = Vec::new();
    let x = 256;
    let y = 240;
    for i in 0..y {
        
        v.push(vec![RGB::create(255.0,0.0,0.0); x]);
    }

    color::write_ppm(v, "test.ppm".to_string());

    println!("Hello, world!");
}
