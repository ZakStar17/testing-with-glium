// todo: move this to lib.rs or something like that

pub use cgmath::{Matrix4, PerspectiveFov};

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

pub trait ToArray {
    type Output;
    fn to_array(&self) -> Self::Output;
}

impl ToArray for Matrix4<f32> {
    type Output = [[f32; 4]; 4];
    fn to_array(&self) -> Self::Output {
        (*self).into()
    }
}

impl ToArray for PerspectiveFov<f32> {
    type Output = [[f32; 4]; 4];
    fn to_array(&self) -> Self::Output {
        let matrix: Matrix4<f32> = (*self).into();
        matrix.to_array()
    }
}

pub type Matrix4Array = [[f32; 4]; 4];

#[allow(dead_code)]
pub fn main() {
    implement_vertex!(Vertex, position, tex_coords);
}