use crate::shaders::common::Vertex3d;
use glium::{Display, VertexBuffer};

use crate::shaders::common::{load_srgb_texture, Material};
use obj::{load_obj, Obj};
use std::fs::File;
use std::io::BufReader;

pub struct KakyoinShader {
    pub vertex_buffer: VertexBuffer<Vertex3d>,
    pub index_buffer: glium::IndexBuffer<u16>,
    pub material: Material,
}

impl KakyoinShader {
    pub fn new(display: &Display) -> Self {
        let input = BufReader::new(File::open("./assets/objects/kakyoin/kakyoin.obj").unwrap());
        let obj: Obj<Vertex3d> = load_obj(input).unwrap();

        Self {
            vertex_buffer: obj.vertex_buffer(display).unwrap(),
            index_buffer: obj.index_buffer(display).unwrap(),
            material: Material {
                diffuse: load_srgb_texture(
                    display,
                    &include_bytes!("../../assets/objects/kakyoin/Kakyoin.png"),
                    image::ImageFormat::Png,
                ),
                specular: load_srgb_texture(
                    display,
                    &include_bytes!("../../assets/black_picture.png"),
                    image::ImageFormat::Png,
                ),
                shininess: 32.0,
            },
        }
    }
}
