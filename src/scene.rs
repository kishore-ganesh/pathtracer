use std::boxed::Box;
use crate::sphere::{Primitive};
use crate::lights::{Light};
use crate::materials::{Material};
pub struct Scene {
   pub primitives: Vec<Primitive>,
   pub light: Box<dyn Light>
}
//Why did Box<dyn Object> not work
impl Scene {
    pub fn create(primitives: Vec<Primitive>, light: Box<dyn Light>) -> Self{
        return Scene{primitives: primitives, light: light};

    }
}
