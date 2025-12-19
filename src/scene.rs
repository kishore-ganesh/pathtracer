use crate::bounding_volume_hierarchy::BVH;
use crate::lights::Light;
pub struct Scene  {
    pub bvh: BVH,
    pub light: Box<dyn Light>,
}
//Why did Box<dyn Object> not work
impl Scene {
    /*pub fn create(primitives: Vec<Primitive>, light: Box<dyn Light + Send>) -> Self{

        return Scene{primitives: primitives, light: light};

    }*/

    pub fn create<'a>(bvh: BVH, light: Box<dyn Light + Send + Sync>) -> Scene {
        Scene {
            bvh,
            light,
        }
    }
}
