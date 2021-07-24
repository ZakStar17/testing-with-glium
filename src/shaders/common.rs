use std::io::Cursor;

use cgmath::{Point3, Vector3};
use glium::{
    texture::{RawImage2d, SrgbTexture2d, Texture2d},
    Display,
};

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
pub struct PositionalVertex {
    pub position: [f32; 3],
}

#[derive(Copy, Clone)]
pub struct Vertex2d {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
}

pub fn load_srgb_texture(
    display: &Display,
    texture_bytes: &dyn std::convert::AsRef<[u8]>,
    image_format: image::ImageFormat,
) -> SrgbTexture2d {
    SrgbTexture2d::new(display, load_raw_image(texture_bytes, image_format)).unwrap()
}

pub fn load_texture(
    display: &Display,
    texture_bytes: &dyn std::convert::AsRef<[u8]>,
    image_format: image::ImageFormat,
) -> Texture2d {
    Texture2d::new(display, load_raw_image(texture_bytes, image_format)).unwrap()
}

fn load_raw_image(
    image_bytes: &dyn std::convert::AsRef<[u8]>,
    image_format: image::ImageFormat,
) -> RawImage2d<u8> {
    use std::time::Instant;

    let now = Instant::now();
    let image = image::load(Cursor::new(image_bytes), image_format)
        .unwrap()
        .to_rgba8();
    println!("Loaded image in {} milliseconds", now.elapsed().as_millis());

    let image_dimensions = image.dimensions();
    RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions)
}

#[allow(dead_code)]
pub fn main() {
    implement_vertex!(Vertex3d, position, normal, tex_coords);
    implement_vertex!(PositionalVertex, position);
    implement_vertex!(Vertex2d, position, tex_coords);
}
