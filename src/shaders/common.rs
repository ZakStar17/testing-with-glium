use cgmath::Vector3;

pub struct Material {
    pub diffuse: glium::Texture2d,
    pub specular: glium::Texture2d,
    pub shininess: f32
}

pub struct Light {
    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
}