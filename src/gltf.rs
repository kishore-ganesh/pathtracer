use std::{cmp::max, path::Path};

use glm::{TMat4, make_mat4x4, make_vec3};
use gltf::{self, Document, Mesh, buffer::Data, camera::{self, Projection}, mesh::util::ReadIndices};

use crate::{bounding_volume_hierarchy::BVH, camera::Camera, color::RGB, lights::SphericalAreaLight, materials::DisneyBRDFMaterial, pathtracer::PathTracer, primitives::{Primitive, transform_mesh}, scene::Scene, sphere::Sphere, triangle_mesh::TriangleMesh};

pub struct GltfLoader {
    document: Document,
    buffers: Vec<Data>
}

#[derive(Debug)]
pub enum Error {
    UnknownError
}


impl GltfLoader {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error>{
        let (document, buffers, _) = gltf::import(path).unwrap();

        Ok(GltfLoader {
            document,
            buffers
        })
    }
    fn load_material(&self,material: &gltf::Material) -> Result<DisneyBRDFMaterial, Error> {
        println!("Material: {}", material.name().unwrap());
        let pbr_metallic_roughness = material.pbr_metallic_roughness();
        println!("PBR base color: {:?} , roughness: {}, metallic: {}", pbr_metallic_roughness.base_color_factor(), pbr_metallic_roughness.roughness_factor(), pbr_metallic_roughness.metallic_factor());
        let specular = material.specular().map(|x| x.specular_factor()).unwrap_or_default();
        // TODO: set minimum roughness set to 0.1
        println!("Specular: color: {:?}", specular);
        // TODO: alpha not handld
        // TODO: specular color factor not handled
        let base_color_factor = pbr_metallic_roughness.base_color_factor();
        let base_color = RGB::create(base_color_factor[0] * 255.0, base_color_factor[1] * 255.0, base_color_factor[2] * 255.0);
        Ok(DisneyBRDFMaterial::create(base_color, pbr_metallic_roughness.metallic_factor(), specular, pbr_metallic_roughness.roughness_factor().max(0.1)))
    }
    fn load_mesh(&self, mesh: &Mesh, transform: &TMat4<f32>) -> Result<Vec<Primitive>, Error> {
        println!("Processing mesh {}", mesh.name().unwrap());
        println!("Mesh length: {}", mesh.primitives().len());
        assert!(mesh.primitives().len() == 1, "Meshes with multiple primitives aren't currently supported");
        let primitive = mesh.primitives().into_iter().next().unwrap();
        let reader = primitive.reader(|buffer| Some(&self.buffers[buffer.index()]));
        // TODO: better error handling
        let positions = reader.read_positions().unwrap().collect();                
        let indices = reader.read_indices().unwrap();
        let indices = match indices {
            ReadIndices::U8(i) => i.map(|x| x as usize).collect(),
            ReadIndices::U16(i) => i.map(|x| x as usize).collect(),
            ReadIndices::U32(i) => i.map(|x| x as usize).collect(),
        };
        let normals = reader.read_normals().unwrap().collect();

        if let Some(positions) = reader.read_positions() {
            println!("Primitive has {} triangles", indices.len() / 3);
        }
        let triangle_mesh = TriangleMesh::from_positions_and_normals(positions, indices, normals);
        let transformed_mesh = transform_mesh(transform, &triangle_mesh);
        let material = self.load_material(&primitive.material())?;
        Ok(Primitive::create_from_mesh(&transformed_mesh, Box::new(material)))
    }
    pub fn load(&self) -> Result<PathTracer, Error> {
        let primitives: Vec<Primitive> = self.document.nodes().flat_map(|node| {
            if let Some(mesh) = node.mesh() {
                // Cannot handle children nodes for now
                assert!(node.children().len() == 0);
                let transform = make_mat4x4(node.transform().matrix().as_flattened());
                return self.load_mesh(&mesh, &transform).unwrap();
            }
            else {
                vec![]
            }
        }).collect();
        let bvh = BVH::create(primitives);
        // TODO: hardcoded area light 
        let sphere = Sphere::create(10.0, make_vec3(&[0.0, 0.0, -100.0]));
        let spherical_area_light = SphericalAreaLight::create(sphere, RGB::create(255.0,255.0,255.0), 100.0);
        let scene = Scene::create(bvh, Box::new(spherical_area_light));
        // TODO: supporting only 1 camera
        let camera_node = self.document.nodes().filter_map(|node| {
            node.camera().map(|_| node)
        }).take(1).next().unwrap();
        let camera = match camera_node.camera().unwrap().projection() {
            Projection::Perspective(perspective) => {
                let transform = make_mat4x4(camera_node.transform().matrix().as_flattened());
                println!("Camera FOV: {}", perspective.yfov().to_degrees());
                println!("Camera transform: {:?}", camera_node.transform().matrix());
                // TODO: not using aspect ratio
                Camera::from_transform(transform, 512.0, 0.1, 1000.0, perspective.yfov())
            }
            Projection::Orthographic(_) => unimplemented!()
        };
        Ok(PathTracer::create(
            512,
            512, 
            1, 
            64*64,
            1.0,
            scene,
            camera
        ))
        
    }
}

// TODO: handling doubleSided