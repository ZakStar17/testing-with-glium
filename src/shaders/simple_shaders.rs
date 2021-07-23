use std::io::Cursor;

use glium::{Display, IndexBuffer, texture::SrgbTexture2d, VertexBuffer};

use crate::common::Vertex;
use crate::shaders::common::Material;
use crate::shaders::shader::ShaderObject;

pub struct CubeShader {
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub index_buffer: IndexBuffer<u8>,
    pub material: Material,
}

impl CubeShader {
    fn get_cube_shape() -> Vec<Vertex> {
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

        let mut vertices: Vec<Vertex> = Vec::with_capacity(36);

        // todo: make this code less ugly
        let mut pos_i = 0;
        let mut tex_i = 0;
        for _ in 0..36 {
            vertices.push(Vertex {
                position: [positions[pos_i], positions[pos_i + 1], positions[pos_i + 2]],
                normal: [normals[pos_i], normals[pos_i + 1], normals[pos_i + 2]],
                tex_coords: [tex_coords[tex_i], tex_coords[tex_i + 1]],
            });

            pos_i += 3;
            tex_i += 2;
        }

        vertices
    }

    fn get_cube_indices() -> [u8; 36] {
        // [
        //     0, 2, 1, 0, 2, 3, 0, 7, 3, 0, 7, 4, 1, 4, 0, 1, 4, 5, 2, 5, 1, 2, 5, 6, 3, 6, 2, 3, 6,
        //     7, 4, 6, 5, 4, 6, 7,
        // ]
        // todo: make this use indices again
        let mut indices = [0u8; 36];
        for i in 0..36 {
            indices[i] = i as u8
        }
        indices
    }
}

impl ShaderObject for CubeShader {
    fn new(display: &Display) -> Self {
        Self {
            vertex_buffer: Self::create_vertex_buffer(display),
            index_buffer: Self::create_index_buffer(display),
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

    fn create_vertex_buffer(display: &Display) -> VertexBuffer<Vertex> {
        let shape = Self::get_cube_shape();
        VertexBuffer::new(display, &shape).unwrap()
    }

    fn create_index_buffer(display: &Display) -> IndexBuffer<u8> {
        let indices = Self::get_cube_indices();

        IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &indices,
        )
        .unwrap()
    }
}

impl CubeShader {
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
