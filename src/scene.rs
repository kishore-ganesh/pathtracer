use crate::bounding_volume_hierarchy::BVHNode;
use crate::lights::Light;
use std::sync::Arc;
#[derive(Clone)]
pub struct Scene<'a> {
    //pub primitives: Vec<Primitive>,
    pub bvh_root: BVHNode<'a>,
    pub light: Arc<dyn Light + Send + Sync>,
}
//Why did Box<dyn Object> not work
impl Scene<'_> {
    /*pub fn create(primitives: Vec<Primitive>, light: Box<dyn Light + Send>) -> Self{

        return Scene{primitives: primitives, light: light};

    }*/

    pub fn create<'a>(bvh_root: BVHNode<'a>, light: Arc<dyn Light + Send + Sync>) -> Scene<'a> {
        Scene {
            bvh_root,
            light,
        }
    }
}
