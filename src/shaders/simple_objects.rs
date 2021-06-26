use std::io::Cursor;

use glium::{Display, VertexBuffer, IndexBuffer, Texture2d};

use crate::common::Vertex;

pub struct CubeObject {
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub index_buffer: IndexBuffer<u8>,
    pub texture: Texture2d,
}

impl CubeObject {
    pub fn new(display: &Display) -> CubeObject {
        CubeObject {
            vertex_buffer: CubeObject::create_vertex_buffer(display),
            index_buffer: CubeObject::create_index_buffer(display),
            texture: CubeObject::load_texture(display),
        }
    }

    pub fn create_vertex_buffer(display: &Display) -> VertexBuffer<Vertex> {
        let shape = CubeObject::get_cube_shape();
        VertexBuffer::new(display, &shape).unwrap()
    }

    fn get_cube_shape() -> Vec<Vertex> {
        vec![
            Vertex {
                position: [-1.0, 1.0, -1.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [-1.0, -1.0, -1.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [1.0, -1.0, -1.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [1.0, 1.0, -1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [-1.0, 1.0, 1.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [-1.0, -1.0, 1.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [1.0, -1.0, 1.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [1.0, 1.0, 1.0],
                tex_coords: [1.0, 1.0],
            },
        ]
    }

    pub fn create_index_buffer(display: &Display) -> IndexBuffer<u8> {
        let indices = CubeObject::get_cube_indices();

        IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &indices,
        )
        .unwrap()
    }

    fn get_cube_indices() -> [u8; 36] {
        [
            0, 2, 1, 0, 2, 3, 0, 7, 3, 0, 7, 4, 1, 4, 0, 1, 4, 5, 2, 5, 1, 2, 5, 6, 3, 6, 2, 3, 6,
            7, 4, 6, 5, 4, 6, 7,
        ]
    }

    pub fn load_texture(display: &Display) -> Texture2d {
        let image = image::load(
            Cursor::new(&include_bytes!("../../assets/wall.png")),
            image::ImageFormat::Png,
        )
        .unwrap()
        .to_rgba8();

        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

            glium::texture::Texture2d::new(display, image).unwrap()
    }
}

