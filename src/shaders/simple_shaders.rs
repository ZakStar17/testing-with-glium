use std::io::Cursor;

use glium::{Display, texture::SrgbTexture2d, VertexBuffer};

use crate::shaders::common::Vertex3d;
use crate::shaders::common::Material;

pub struct CubeShader {
    pub vertex_buffer: VertexBuffer<Vertex3d>,
    pub index_buffer: glium::index::NoIndices,
    pub material: Material,
}

impl CubeShader {
    pub fn new(display: &Display) -> Self {
        Self {
            vertex_buffer: Self::create_vertex_buffer(display),
            index_buffer: glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            material: Material {
                diffuse: Self::load_texture(
                    display,
                    &include_bytes!("../../assets/container2.png"),
                ),
                specular: Self::load_texture(
                    display,
                    &include_bytes!("../../assets/container2_specular.png"),
                ),
                shininess: 32.0,
            },
        }
    }


    fn get_cube_shape() -> Vec<Vertex3d> {
        let positions = [
            -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0, -1.0,
            -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0,
            1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, -1.0,
            -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0,
            -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, -1.0,
            -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0,
            -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0,
        ];
        let normals = [
            0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0,
            0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
            0.0, 0.0, 1.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0,
            0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
            1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0,
            -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
            0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
        ];
        let tex_coords = [
            0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0,
            1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0,
            1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0,
            1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
        ];

        let mut vertices: Vec<Vertex3d> = Vec::with_capacity(36);

        // todo: make this code less ugly
        let mut pos_i = 0;
        let mut tex_i = 0;
        for _ in 0..36 {
            vertices.push(Vertex3d {
                position: [positions[pos_i], positions[pos_i + 1], positions[pos_i + 2]],
                normal: [normals[pos_i], normals[pos_i + 1], normals[pos_i + 2]],
                tex_coords: [tex_coords[tex_i], tex_coords[tex_i + 1]],
            });

            pos_i += 3;
            tex_i += 2;
        }

        vertices
    }

    fn create_vertex_buffer(display: &Display) -> VertexBuffer<Vertex3d> {
        let shape = Self::get_cube_shape();
        VertexBuffer::new(display, &shape).unwrap()
    }

    fn load_texture(display: &Display, texture_bytes: &dyn std::convert::AsRef<[u8]>) -> SrgbTexture2d {
        let image = image::load(Cursor::new(texture_bytes), image::ImageFormat::Png)
            .unwrap()
            .to_rgba8();

        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        SrgbTexture2d::new(display, image).unwrap()
    }
}
