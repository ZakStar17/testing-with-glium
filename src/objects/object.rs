
use cgmath::{EuclideanSpace, Euler, Matrix4, Point3, Rad};

/// Simple object without non-uniform scaling.
/// Implements position, rotation and scale
pub trait Object {
    fn new(position: Point3<f32>) -> Self;

    fn from_full(position: Point3<f32>, rotation: Euler<Rad<f32>>, scale: f32) -> Self;

    fn update_model(&mut self);
}


pub fn create_model_matrix(
    position: Point3<f32>,
    rotation: Euler<Rad<f32>>,
    scale: f32,
) -> Matrix4<f32> {
    let translation_matrix = Matrix4::from_translation(position.to_vec());
    let rotation_matrix = Matrix4::from(rotation);
    let scale_matrix = Matrix4::from_scale(scale);

    translation_matrix * rotation_matrix * scale_matrix
}