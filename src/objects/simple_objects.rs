use cgmath::{Euler, Point3, Rad};

use crate::objects::renderable_3d_object::Renderable3dObject;
use crate::shaders::common::PointLight;

pub struct Cube {
    pub object: Renderable3dObject,
}

impl Cube {
    pub fn new(position: Point3<f32>) -> Cube {
        let object = Renderable3dObject::new(position);
        Cube { object }
    }

    pub fn from_full(position: Point3<f32>, rotation: Euler<Rad<f32>>, scale: f32) -> Cube {
        Cube {
            object: Renderable3dObject::from_full(position, rotation, scale),
        }
    }
}

pub struct SimpleLightCube {
    pub object: Renderable3dObject,
    pub light: PointLight,
}

impl SimpleLightCube {
    pub fn new(rotation: Euler<Rad<f32>>, scale: f32, light: PointLight) -> Self {
        SimpleLightCube {
            object: Renderable3dObject::from_full(light.position, rotation, scale),
            light,
        }
    }
}
