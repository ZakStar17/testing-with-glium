use cgmath::{EuclideanSpace, Euler, Matrix4, Point3, Rad, Vector3};

use crate::objects::object::{create_model_matrix, Object};

pub struct Cube {
    pub position: Point3<f32>,
    pub rotation: Euler<Rad<f32>>,
    pub scale: f32,
    pub model_matrix: Matrix4<f32>,
}

impl Object for Cube {
    fn new(position: Point3<f32>) -> Cube {
        let model_matrix = Matrix4::from_translation(position.to_vec());

        Cube {
            position,
            rotation: Euler {
                x: Rad(0.0),
                y: Rad(0.0),
                z: Rad(0.0),
            },
            scale: 1.0,
            model_matrix: model_matrix,
        }
    }

    fn from_full(position: Point3<f32>, rotation: Euler<Rad<f32>>, scale: f32) -> Cube {
        let model_matrix = create_model_matrix(position, rotation, scale);

        Cube {
            position,
            rotation,
            scale,
            model_matrix,
        }
    }

    fn update_model(&mut self) {
        self.model_matrix = create_model_matrix(self.position, self.rotation, self.scale)
    }
}

pub struct SimpleLightCube {
    pub cube: Cube,
    pub light_color: Vector3<f32>,
}

impl SimpleLightCube {
    pub fn new(cube: Cube, light_color: Vector3<f32>) -> Self {
        SimpleLightCube { cube, light_color }
    }
}
