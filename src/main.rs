extern crate nalgebra_glm as glm;
extern crate rand;
extern crate obj as obj_parser_external;
mod sphere;
mod color;
mod cube;
mod lights;
mod materials;
mod sampler;
mod scene;
mod camera;
mod obj_parser;
mod primitives;
mod pathtracer;
mod plane;
mod triangle;
mod triangle_mesh;
mod bounding_box;
mod bounding_volume_hierarchy;


use std::f32::consts::PI;
use glm::{dot, TMat4, TVec4, make_mat4x4, make_vec4, transpose, make_vec3, vec3_to_vec4, mat4_to_mat3};
use cube::{create_cube};
use sphere::{Sphere, Ray, Object, Primitive};
use primitives::{Rect, reflect_about_vec, rotate_about_x, rotate_about_y, get_perp_vec, scale, translate, transform, transform_mesh, transform_vec};
use color::RGB;
use pathtracer::PathTracer;
use scene::Scene;
use camera::Camera;
use lights::{PointLight, SphericalAreaLight};
use obj_parser::parse;
use triangle::{NormalType, Triangle};
use triangle_mesh::TriangleMesh;
use materials::{DiffuseMaterial, SpecularMaterial, DisneyBRDFMaterial};
use plane::Plane;
use bounding_box::{BoundingBox};
use bounding_volume_hierarchy::BVHNode;
use std::sync::Arc;
fn main() {
    // let imported_cube_mesh = parse(make_vec3(&[0.0,0.0,0.0]), "models/cube.obj");
    //println!("a: 123.0, b: nan, min(a, b): {}. max(a, b): {}", float_min(123.0, f32::NAN), float_max(123.0, f32::NAN));
    let bounding_box = BoundingBox::create(
        make_vec3(&[0.0,0.0,0.0],
        ),
        make_vec3(&[5.0,5.0,5.0],
        )
        
    );

    let ray = Ray::create(
        make_vec3(&[0.0,0.0,0.0]),
        make_vec3(&[1.0,1.0,1.0])
    );

    println!("Intersection with box is: {}", bounding_box.intersection(&ray));


    //panic!("Bounding box test");
    
    let suzanne_mesh = parse(make_vec3(&[0.0,0.0,0.0]), "models/suzanne.obj");
    // let teapot_mesh = parse(make_vec3(&[0.0,0.0,0.0]), "models/teapot.obj");
    //let max_planck_mesh = parse(make_vec3(&[0.0,0.0,0.0]), "models/max_planck.obj");
    //let gopher_mesh = parse(make_vec3(&[0.0,0.0,0.0]), "models/gopher.obj");
    
    //let imported_tri_mesh = parse(make_vec3(&[0.0,0.0,0.0]), "models/triangle.obj");
    let transformed_suzanne_mesh = transform_mesh(&(translate(0.0,0.0,0.0) * scale(2.0,2.0,2.0)), &suzanne_mesh);
    // let transformed_teapot_mesh = transform_mesh(&(translate(0.0,0.0,-5.0) * scale(10.0,10.0,10.0)), &teapot_mesh);
    //let transformed_max_planck_mesh = transform_mesh(&(translate(0.0,0.0,-5.0) * scale(10.0,10.0,10.0)), &max_planck_mesh);
    //println!("Imported Cube mesh: {:?}", imported_cube_mesh);
    //println!("Transformed Suzanne mesh: {:?}", transformed_suzanne_mesh);
    let v1 = make_vec3(&[0.0,1.0,0.0]); 
    let v2 = make_vec3(&[1.2, 0.312, 2.4]);
    //println!("perp vec to: {} is: {}", &v1, get_perp_vec(&v1));
    //println!("perp vec to: {} is: {}, dot is: {}", &v2, get_perp_vec(&v2), dot(&v2, &get_perp_vec(&v2)));

    let center = make_vec3(&[ 0.0,1.0,0.0 ]);
    let x: Sphere = Sphere::create(1.0, center.clone());
    let r: Ray = Ray{origin: make_vec3(&[ 1.0,1.0,1.0 ]), direction: make_vec3(&[1.0,1.0,1.0])};
    x.intersection(&r);
    let v = make_vec3(&[-1.0,1.0,0.0]);
    let normal = make_vec3(&[0.0,1.0,0.0]);
    let (reflected_v) = reflect_about_vec(&v, &normal);
    //println!("original: {}, reflected: {}", v, reflected_v);
    let rotate_angle = (0.0) * (PI/180.0);
    let cube_mesh = create_cube(make_vec3(&[ 0.0,45.0,30.0 ]), rotate_angle, 0.0, 50.0, true);
    println!("Cube mesh is: {:?}", cube_mesh);
    let plane_rotate_angle = (50.0) * (PI/180.0);
    //let p1_vec = transform_vec(&rotate_about_x(plane_rotate_angle), &make_vec3(&[0.0,0.0,1.0]));
    let p1_vec = make_vec3(&[0.0,1.0,0.0]);
    let p2_vec = make_vec3(&[0.0,0.0,1.0]);
    let p3_vec = make_vec3(&[1.0,0.0,0.0]);
    let p4_vec = make_vec3(&[-1.0,0.0,0.0]);
    let p5_vec = make_vec3(&[0.0,-1.0,0.0]);
    let p1 = Plane::create_point_normal(make_vec3(&[ 0.0,-2.0,0.0 ]), p1_vec);
    let p2 = Plane::create_point_normal(make_vec3(&[ 0.0,0.0,-10.0 ]), p2_vec);
    let p3 = Plane::create_point_normal(make_vec3(&[ -5.0,0.0,0.0 ]), p3_vec);
    let p4 = Plane::create_point_normal(make_vec3(&[ 5.0,0.0,0.0 ]), p4_vec);
    let p5 = Plane::create_point_normal(make_vec3(&[ 0.0,6.0,0.0 ]), p5_vec);
    
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
    //println!("{:?}", d);
    let b_3 = mat4_to_mat3(&b);
    //println!("{:?} {:?}", b, b_3);
    ////println!("{:?}", transpose(&a) * b);
    //color::write_ppm(&v, "test.ppm".to_string());
    //
    */
    //TODO: link up sphere and cam?
    //Need small region scale to control distortion
    let screen_res = 1280.0;
    let raster_res = 1280.0;
    let look_at_point = make_vec3(&[ 0.0,0.0,0.0 ]);
    let region_scale = 1.0;
    let fov = 60.0;
    let point_light = Arc::new(PointLight::create(
            make_vec3(&[ 0.0, 20.0,2.0 ]),
            RGB::create(255.0,255.0,255.0),
            3.0
            ));
    let spherical_area_light = Arc::new(SphericalAreaLight::create(
        Sphere::create(10.0, make_vec3(&[0.0,0.0,20.0])),
        RGB::create(255.0,255.0,255.0),
        10.0
    ));
    let n_samples = 1024;
    let chunk_size = 16;
    //println!("Chunk size: {}", chunk_size);
    let roulette_threshold = 0.01;
    let region = Rect::create(make_vec3(&[ -region_scale,-region_scale,0.0 ]), make_vec3(&[ region_scale, region_scale,0.0 ]));
//    let look_at_point = make_vec3(&[ 0.0,0.0,1.0 ]);
    let camera = Camera::look_at(make_vec3(&[ 0.0,0.0,10.0 ]), look_at_point, 0.1, 1000.0, screen_res, raster_res, fov,region);
    let relative_point = transform(&camera.camera_to_world, &make_vec3(&[ 0.0,0.0,50.0 ]));
    // let x2: Sphere = Sphere::crea te(1.0, make_vec3(&[ 1.0,-1.0,0.0 ]));
    // let x3: Sphere = Sphere::create(1.0, make_vec3(&[-1.0, -1.0, 3.0]));
    let x2: Sphere = Sphere::create(1.0, make_vec3(&[ 1.0,-1.0, 2.0 ]));
    let x3: Sphere = Sphere::create(1.0, make_vec3(&[ -1.0,-1.0, 3.0 ]));
    
    let triangle = Triangle::create([
                                    make_vec3(&[ -2.0,0.0,0.0 ]),
                                    make_vec3(&[ 0.0, 2.0,0.0 ]),
                                    make_vec3(&[ 2.0,-2.0,0.0 ])
    ],
     NormalType::FaceNormal(make_vec3(&[0.0,0.0,1.0]) 
    )); 

    let diffuse_material = DiffuseMaterial::create(RGB::create(0.0,255.0,127.0)); 
    let red_diffuse_material = DiffuseMaterial::create(RGB::create(255.0,0.0, 0.0));
    let green_diffuse_material = DiffuseMaterial::create(RGB::create(0.0,255.0,0.0));
    let blue_diffuse_material = DiffuseMaterial::create(RGB::create(0.0,0.0,255.0));
    let white_diffuse_material = DiffuseMaterial::create(RGB::create(255.0,255.0,255.0));
    let specular_material = SpecularMaterial::create();

    let disney_diffuse_material = DisneyBRDFMaterial::create(RGB::create(255.0,120.0,120.0), 0.0,0.01,0.8); //NOTE: names may no longer match material properties
    let disney_violet_diffuse_material = DisneyBRDFMaterial::create(RGB::create(40.0, 12.0, 148.0), 0.0,0.9,0.2);
    let disney_white_diffuse_material = DisneyBRDFMaterial::create(RGB::create(255.0, 255.0, 255.0), 0.0,0.9,0.02);
    let disney_red_diffuse_material = DisneyBRDFMaterial::create(RGB::create(255.0,0.0,0.0), 0.0,0.9,0.5);
    let disney_green_diffuse_material = DisneyBRDFMaterial::create(RGB::create(0.0,255.0,0.0), 0.0,0.0,0.5);
    let disney_blue_diffuse_material = DisneyBRDFMaterial::create(RGB::create(0.0,0.0,255.0), 0.0,0.0,0.5);
    let disney_yellow_diffuse_material = DisneyBRDFMaterial::create(RGB::create(255.0,255.0, 0.0), 0.0,0.0,0.5);
    let disney_glossy_material = DisneyBRDFMaterial::create(RGB::create(255.0,255.0, 0.0), 0.0,0.5,0.5);
    let disney_silver_material = DisneyBRDFMaterial::create(RGB::create(211.0,211.0,211.0), 0.2, 0.9,0.02);
    let mut mesh_primitives = vec![
        // Primitive::create_from_mesh(&transformed_suzanne_mesh, Arc::new(disney_glossy_material.clone())),
        // Primitive::create_from_mesh(&transformed_teapot_mesh, Arc::new(disney_glossy_material.clone())),
        //Primitive::create_from_mesh(&transformed_max_planck_mesh, Arc::new(disney_glossy_material.clone()))
        //Primitive::create_from_mesh(&gopher_mesh, Arc::new(disney_glossy_material.clone()))
        Primitive::create_from_mesh(&cube_mesh, Arc::new(disney_diffuse_material.clone()))
        //Primitive::create_from_mesh(&imported_cube_mesh, Arc::new(disney_diffuse_material.clone()))
    ];
    let mut other_primitives = 
        vec![
            
            
            //Primitive::create(Arc::new(imported_tri_mesh), Arc::new(diffuse_material.clone())),
            // Primitive::create(Arc::new(cube), Arc::new(disney_diffuse_material.clone())),
             Primitive::create(Arc::new(x), Arc::new(disney_glossy_material.clone())),
             Primitive::create(Arc::new(x2), Arc::new(disney_white_diffuse_material.clone())),
             Primitive::create(Arc::new(x3), Arc::new(disney_red_diffuse_material.clone())),
            //  Primitive::create(spherical_area_light.clone(), Arc::new(white_diffuse_material.clone())),
            //Primitive::create(Arc::new(x), Arc::new(diffuse_material.clone())),
             //Primitive::create(Arc::new(cube), Arc::new(diffuse_material.clone())),
            // Primitive::create(Arc::new(p1), Arc::new(disney_white_diffuse_material.clone())),
          //   Primitive::create(Arc::new(p2), Arc::new(disney_red_diffuse_material.clone())),

          
            //Primitive::create(Arc::new(p3), Arc::new(disney_green_diffuse_material.clone())),
            //Primitive::create(Arc::new(p4), Arc::new(disney_blue_diffuse_material.clone())),
            //Primitive::create(Arc::new(p5), Arc::new(disney_yellow_diffuse_material.clone())),
            

            //Arc::new(triangle),
            //Arc::new(x),
            //Arc::new(x2)

        ];
    

    for mesh_primitive in &mut mesh_primitives {
        other_primitives.append(mesh_primitive);
    }
    let bvh_root = BVHNode::create(
        &other_primitives
    );

    bvh_root.print_traverse();
    //panic!("Before scene creation");
    let scene = Scene::create(bvh_root, spherical_area_light);

    let mut pt = PathTracer::create(raster_res as i32, raster_res as i32, n_samples, chunk_size, roulette_threshold, scene, camera);
    let grid = pt.generate(); 
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            //  println!("Value at {} {} is: {:?}", row, col, grid[row][col])
        }
    }
    color::write_ppm(&grid, "test.ppm".to_string());
    ////println!("Hello, world!");
}
