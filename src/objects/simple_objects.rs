use cgmath::{EuclideanSpace, Euler, Matrix4, Point3, Rad};

use crate::objects::object::{create_model_matrix, Object};
use crate::shaders::common::Light;

pub struct Cube {
    pub object: Object,
}

impl Cube {
    pub fn new(position: Point3<f32>) -> Cube {
        let model_matrix = Matrix4::from_translation(position.to_vec());

        let object = Object {
            position,
            rotation: Euler {
                x: Rad(0.0),
                y: Rad(0.0),
                z: Rad(0.0),
            },
            scale: 1.0,
            model_matrix: model_matrix,
        };

        Cube { object }
    }

    pub fn from_full(position: Point3<f32>, rotation: Euler<Rad<f32>>, scale: f32) -> Cube {
        let model_matrix = create_model_matrix(position, rotation, scale);

        Cube {
            object: Object {
                position,
                rotation,
                scale,
                model_matrix,
            },
        }
    }
}

pub struct SimpleLightCube {
    pub object: Object,
    pub light: Light,
}

impl SimpleLightCube {
    pub fn new(position: Point3<f32>, rotation: Euler<Rad<f32>>, scale: f32, light: Light) -> Self {
        let model_matrix = create_model_matrix(position, rotation, scale);

        SimpleLightCube {
            object: Object {
                position,
                rotation,
                scale,
                model_matrix,
            },
            light,
        }
    }
}
