extern crate nalgebra_glm as glm;
extern crate rand;
mod sphere;
mod color;
mod cube;
mod lights;
mod materials;
mod sampler;
mod scene;
mod camera;
mod primitives;
mod pathtracer;
mod plane;
mod triangle;
mod triangle_mesh;


use std::f32::consts::PI;
use glm::{TMat4, TVec4, make_mat4x4, make_vec4, transpose, make_vec3, vec3_to_vec4, mat4_to_mat3};
use cube::Cube;
use sphere::{Sphere, Ray, Object, Primitive};
use primitives::{Point, Rect, transform, transform_mesh, transform_vec, scale,reflect_about_vec, rotate_about_x, rotate_about_y};
use color::RGB;
use pathtracer::PathTracer;
use scene::Scene;
use camera::Camera;
use lights::PointLight;
use triangle::Triangle;
use triangle_mesh::TriangleMesh;
use materials::{DiffuseMaterial, SpecularMaterial};
use plane::Plane;
fn main() {
    let center = Point::create(-1.0,0.0,0.0);
    let x: Sphere = Sphere::create(1.0, center.clone());
    let r: Ray = Ray{origin: Point::create(1.0,1.0,1.0), direction: make_vec3(&[1.0,1.0,1.0])};
    x.intersection(&r);
    let v = make_vec3(&[-1.0,1.0,0.0]);
    let normal = make_vec3(&[0.0,1.0,0.0]);
    let (reflected_v, _) = reflect_about_vec(&v, &normal);
    println!("original: {}, reflected: {}", v, reflected_v);
    let rotate_angle = (45.0) * (PI/180.0);
    let cube = Cube::create(Point::create(0.0,-100.0,0.0), rotate_angle, 0.0, 100.0, true);
    let plane_rotate_angle = (50.0) * (PI/180.0);
    //let p1_vec = transform_vec(&rotate_about_x(plane_rotate_angle), &make_vec3(&[0.0,0.0,1.0]));
    let p1_vec = make_vec3(&[0.0,1.0,0.0]);
    let p2_vec = make_vec3(&[0.0,1.0,1.0]);
    let p3_vec = make_vec3(&[1.0,0.0,0.0]);
    let p4_vec = make_vec3(&[-1.0,0.0,0.0]);
    let p1 = Plane::create_point_normal(Point::create(0.0,-2.0,0.0), p1_vec);
    let p2 = Plane::create_point_normal(Point::create(0.0,0.0,-10.0), p2_vec);
    let p3 = Plane::create_point_normal(Point::create(-5.0,0.0,0.0), p3_vec);
    let p4 = Plane::create_point_normal(Point::create(5.0,0.0,0.0), p4_vec);
    
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
    //Need small region scale to control distortion
    let screen_res = 512.0;
    let raster_res = 512.0;
    let look_at_point = Point::create(0.0,0.0,0.0);
    let region_scale = 1.0;
    let fov = 60.0;
    let point_light = Box::new(PointLight::create(
            Point::create(0.0, 0.0,2.0),
            RGB::create(255.0,255.0,255.0),
            10.0
    ));

    let n_samples = 1;
    let region = Rect::create(Point::create(-region_scale,-region_scale,0.0), Point::create(region_scale, region_scale,0.0));
//    let look_at_point = Point::create(0.0,0.0,1.0);
    let camera = Camera::look_at(Point::create(0.0,0.0,10.0), look_at_point, 0.1, 1000.0, screen_res, raster_res, fov,region);
    let relative_point = transform(&camera.camera_to_world, &Point::create(0.0,0.0,50.0));
    let x2: Sphere = Sphere::create(1.0, Point::create(1.0,0.0,0.0));
    let triangle = Triangle::create([
                                    Point::create(-2.0,0.0,0.0),
                                    Point::create(0.0, 2.0,0.0),
                                    Point::create(2.0,-2.0,0.0)
    ],
     make_vec3(&[0.0,0.0,1.0]) 
    ); 

    let diffuse_material = DiffuseMaterial::create(RGB::create(0.0,255.0,127.0)); 
    let red_diffuse_material = DiffuseMaterial::create(RGB::create(255.0,0.0, 0.0));
    let green_diffuse_material = DiffuseMaterial::create(RGB::create(0.0,255.0,0.0));
    let blue_diffuse_material = DiffuseMaterial::create(RGB::create(0.0,0.0,255.0));
    let specular_material = SpecularMaterial::create();
    let scene = Scene::create(vec![
                              Primitive::create(Box::new(x), Box::new(specular_material.clone())),
                              //Primitive::create(Box::new(x2), Box::new(diffuse_material.clone())),
                              //Primitive::create(Box::new(cube), Box::new(diffuse_material.clone())),
                              Primitive::create(Box::new(p1), Box::new(diffuse_material.clone())),
                              Primitive::create(Box::new(p2), Box::new(red_diffuse_material.clone())),
                              Primitive::create(Box::new(p3), Box::new(green_diffuse_material.clone())),
                              Primitive::create(Box::new(p4), Box::new(blue_diffuse_material.clone())),
                              

                              //Box::new(triangle),
                              //Box::new(x),
                              //Box::new(x2)

    ], point_light);

    let mut pt = PathTracer::create(raster_res as i32, raster_res as i32, n_samples, 1.0, scene, camera);
    let grid = pt.generate(); 
    color::write_ppm(&grid, "test.ppm".to_string());
    //println!("Hello, world!");
}
