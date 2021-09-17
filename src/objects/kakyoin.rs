use cgmath::{EuclideanSpace, Euler, Matrix4, Point3, Rad};

use crate::objects::object::{create_model_matrix, Object};

pub struct Kakyoin {
    pub object: Object,
}

impl Kakyoin {
    pub fn new(position: Point3<f32>) -> Self {
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

        Self { object }
    }

    pub fn from_full(position: Point3<f32>, rotation: Euler<Rad<f32>>, scale: f32) -> Self {
        let model_matrix = create_model_matrix(position, rotation, scale);

        Self {
            object: Object {
                position,
                rotation,
                scale,
                model_matrix,
            },
        }
    }
}
