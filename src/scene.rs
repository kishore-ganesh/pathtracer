use std::boxed::Box;
use crate::sphere::{Object};

pub struct Scene {
   pub objects: Vec<Box<dyn Object>> 
}
//Why did Box<dyn Object> not work
impl Scene {
    pub fn create(objects: Vec<Box<dyn Object>>) -> Self{
        return Scene{objects: objects};

    }
}
