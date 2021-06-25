use std::io::Cursor;

use cgmath::{EuclideanSpace, Euler, Matrix4, Point3, Rad};
use glium;

use crate::common::{Matrix4Array, ToArray, Vertex};

pub struct Cube {
    pub position: Point3<f32>,
    pub rotation: Euler<Rad<f32>>,
    pub scale: f32,
    pub model_matrix: Matrix4Array,
}

impl Cube {
    pub fn new(position: Point3<f32>) -> Cube {
        let model_matrix = Matrix4::from_translation(position.to_vec()).to_array();

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

pub struct CubeConfig {
    pub vertex_buffer: glium::VertexBuffer<Vertex>,
    pub index_buffer: glium::IndexBuffer<u8>,
    pub program: glium::Program,
    pub texture: glium::Texture2d,
}

impl CubeConfig {
    pub fn new(display: &glium::Display) -> CubeConfig {
        CubeConfig {
            vertex_buffer: CubeConfig::create_vertex_buffer(display),
            index_buffer: CubeConfig::create_index_buffer(display),
            program: CubeConfig::load_program(display),
            texture: CubeConfig::load_texture(display)
        }
    }

    pub fn create_vertex_buffer(display: &glium::Display) -> glium::VertexBuffer<Vertex> {
        let shape = CubeConfig::get_cube_shape();
        glium::VertexBuffer::new(display, &shape).unwrap()
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

    pub fn create_index_buffer(display: &glium::Display) -> glium::IndexBuffer<u8> {
        let indices = CubeConfig::get_cube_indices();

        glium::IndexBuffer::new(
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

    pub fn load_program(display: &glium::Display) -> glium::Program {
        let vertex_shader_src = r#"
            #version 140

            in vec3 position;
            in vec2 tex_coords;

            out vec2 v_tex_coords;

            uniform mat4 model;
            uniform mat4 view;
            uniform mat4 projection;

            void main() {
                gl_Position = projection * view * model * vec4(position, 1.0);
                v_tex_coords = tex_coords;
            }
        "#;

        let fragment_shader_src = r#"
            #version 140

            in vec2 v_tex_coords;
            out vec4 color;
            
            uniform sampler2D tex;
            
            void main() {
                color = ((texture(tex, v_tex_coords) * vec4(8.0, 8.0, 8.0, 1.0)) - vec4(1.9, 1.9, 1.9, 0.0)) * vec4(0.125, 0.125, 0.125, 1.0);
            }
        "#;

        glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap()
    }

    pub fn load_texture(display: &glium::Display) -> glium::Texture2d {
        let image = image::load(
            Cursor::new(&include_bytes!("./wall.png")),
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

fn create_model_matrix(
    position: Point3<f32>,
    rotation: Euler<Rad<f32>>,
    scale: f32,
) -> Matrix4Array {
    let translation_matrix = Matrix4::from_translation(position.to_vec());
    let rotation_matrix = Matrix4::from(rotation);
    let scale_matrix = Matrix4::from_scale(scale);

    (translation_matrix * rotation_matrix * scale_matrix).to_array()
}
