mod sphere;
use sphere::{Sphere, Ray, Point};
fn main() {
    let x: Sphere = Sphere{r: 10};
    let r: Ray = Ray{origin: Point{x:1, y:1, z:1}, direction: Point{x:1, y:1, z:1}};
    x.intersection(r);
    println!("Hello, world!");
}
