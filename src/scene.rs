use std::boxed::Box;
use crate::sphere::{Object};
use crate::lights::{Light};
pub struct Scene {
   pub objects: Vec<Box<dyn Object>>,
   pub light: Box<dyn Light>
}
//Why did Box<dyn Object> not work
impl Scene {
    pub fn create(objects: Vec<Box<dyn Object>>, light: Box<dyn Light>) -> Self{
        return Scene{objects: objects, light: light};

    }
}
