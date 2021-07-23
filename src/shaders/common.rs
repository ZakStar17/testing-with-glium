use cgmath::{Point3, Vector3};

pub struct Material {
    pub diffuse: glium::texture::SrgbTexture2d,
    pub specular: glium::texture::SrgbTexture2d,
    pub shininess: f32,
}

#[derive(Clone)]
pub struct DirectionalLight {
    pub direction: Vector3<f32>,

    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
}

#[derive(Clone)]
pub struct PointLight {
    pub position: Point3<f32>,

    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,

    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}

#[derive(Clone)]
pub struct SpotLight {
    pub position: Point3<f32>,
    pub direction: Vector3<f32>,
    pub cut_off: f32,
    pub outer_cut_off: f32,

    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
}

#[derive(Copy, Clone)]
pub struct Vertex3d {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}

#[derive(Copy, Clone)]
pub struct Vertex2d {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}

#[allow(dead_code)]
pub fn main() {
    implement_vertex!(Vertex3d, position, normal, tex_coords);
    implement_vertex!(Vertex2d, position, tex_coords);
}
