use cgmath::{EuclideanSpace, Euler, Matrix4, Point3, Rad};

pub struct Cube {
    pub position: Point3<f32>,
    pub rotation: Euler<Rad<f32>>,
    pub scale: f32,
    pub model_matrix: Matrix4<f32>,
}

impl Cube {
    pub fn new(position: Point3<f32>) -> Cube {
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

    pub fn from_full(position: Point3<f32>, rotation: Euler<Rad<f32>>, scale: f32) -> Cube {
        let model_matrix = create_model_matrix(position, rotation, scale);

        Cube {
            position,
            rotation,
            scale,
            model_matrix,
        }
    }

    pub fn update_model(&mut self) {
        self.model_matrix = create_model_matrix(self.position, self.rotation, self.scale)
    }
}

fn create_model_matrix(
    position: Point3<f32>,
    rotation: Euler<Rad<f32>>,
    scale: f32,
) -> Matrix4<f32> {
    let translation_matrix = Matrix4::from_translation(position.to_vec());
    let rotation_matrix = Matrix4::from(rotation);
    let scale_matrix = Matrix4::from_scale(scale);

    translation_matrix * rotation_matrix * scale_matrix
}
