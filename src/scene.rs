use std::boxed::Box;
use crate::sphere::{Primitive};
use crate::lights::{Light};
use crate::materials::{Material};
use crate::bounding_volume_hierarchy::BVHNode;
use std::sync::Arc;
#[derive(Clone)]
pub struct Scene {
   //pub primitives: Vec<Primitive>,
   pub bvh_root: BVHNode,
   pub light: Arc<dyn Light + Send + Sync>
}
//Why did Box<dyn Object> not work
impl Scene {
    /*pub fn create(primitives: Vec<Primitive>, light: Box<dyn Light + Send>) -> Self{
        
        return Scene{primitives: primitives, light: light};

    }*/

    pub fn create(bvh_root: BVHNode, light: Arc<dyn Light + Send + Sync>) -> Self{
        return Scene{bvh_root: bvh_root, light: light};
    }
}
