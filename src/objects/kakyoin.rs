use crate::objects::renderable_3d_object::HasRenderable3dObject;
use crate::objects::renderable_3d_object::Renderable3dObject;
use cgmath::{Euler, Point3, Rad};

pub struct Kakyoin {
    pub object: Renderable3dObject,
}

impl Kakyoin {
    pub fn new(position: Point3<f32>) -> Self {
        Self {
            object: Renderable3dObject::new(position),
        }
    }

    pub fn from_full(position: Point3<f32>, rotation: Euler<Rad<f32>>, scale: f32) -> Self {
        Self {
            object: Renderable3dObject::from_full(position, rotation, scale),
        }
    }
}

impl HasRenderable3dObject for Kakyoin {
    fn get_object(&self) -> &'_ Renderable3dObject {
        &self.object
    }

    fn get_object_mut(&mut self) -> &'_ mut Renderable3dObject {
        &mut self.object
    }
}
