use glium::{
    framebuffer::SimpleFrameBuffer,
    texture::{cubemap::Cubemap, CubeLayer, Texture2d},
    Display, IndexBuffer, Surface, VertexBuffer,
};

use crate::shaders::common::{load_texture, PositionalVertex};

pub struct CubeMapShader {
    pub vertex_buffer: VertexBuffer<PositionalVertex>,
    pub index_buffer: IndexBuffer<u16>,
    pub cubemap: Cubemap,
}

impl CubeMapShader {
    pub fn new(display: &Display) -> Self {
        CubeMapShader {
            vertex_buffer: Self::create_vertex_buffer(display),
            index_buffer: Self::create_index_buffer(display),
            cubemap: Self::load_cubemap(display),
        }
    }

    fn load_textures(display: &Display) -> [glium::Texture2d; 6] {
        [
            load_texture(
                display,
                &include_bytes!("../../assets/skybox/right.jpg"),
                image::ImageFormat::Jpeg,
            ),
            load_texture(
                display,
                &include_bytes!("../../assets/skybox/left.jpg"),
                image::ImageFormat::Jpeg,
            ),
            load_texture(
                display,
                &include_bytes!("../../assets/skybox/top.jpg"),
                image::ImageFormat::Jpeg,
            ),
            load_texture(
                display,
                &include_bytes!("../../assets/skybox/bottom.jpg"),
                image::ImageFormat::Jpeg,
            ),
            load_texture(
                display,
                &include_bytes!("../../assets/skybox/front.jpg"),
                image::ImageFormat::Jpeg,
            ),
            load_texture(
                display,
                &include_bytes!("../../assets/skybox/back.jpg"),
                image::ImageFormat::Jpeg,
            ),
        ]
    }

    fn load_cubemap(display: &Display) -> Cubemap {
        let textures = Self::load_textures(display);
        let size = textures[0].width();

        let cubemap = glium::texture::Cubemap::empty(display, size).unwrap();

        let create_framebuffer_from =
            |layer| SimpleFrameBuffer::new(display, cubemap.main_level().image(layer)).unwrap();

        let framebuffer_pos_x = create_framebuffer_from(CubeLayer::PositiveX);
        let framebuffer_neg_x = create_framebuffer_from(CubeLayer::NegativeX);
        let framebuffer_pos_y = create_framebuffer_from(CubeLayer::PositiveY);
        let framebuffer_neg_y = create_framebuffer_from(CubeLayer::NegativeY);
        let framebuffer_pos_z = create_framebuffer_from(CubeLayer::PositiveZ);
        let framebuffer_neg_z = create_framebuffer_from(CubeLayer::NegativeZ);

        let fill_framebuffer = |texture: &Texture2d, framebuffer| {
            texture
                .as_surface()
                .fill(framebuffer, glium::uniforms::MagnifySamplerFilter::Linear)
        };

        fill_framebuffer(&textures[0], &framebuffer_pos_x);
        fill_framebuffer(&textures[1], &framebuffer_neg_x);
        fill_framebuffer(&textures[2], &framebuffer_pos_y);
        fill_framebuffer(&textures[3], &framebuffer_neg_y);
        fill_framebuffer(&textures[4], &framebuffer_pos_z);
        fill_framebuffer(&textures[5], &framebuffer_neg_z);

        cubemap
    }

    fn create_vertex_buffer(display: &Display) -> VertexBuffer<PositionalVertex> {
        let side: f32 = 1.0;

        VertexBuffer::new(
            display,
            &[
                // Front
                PositionalVertex {
                    position: [-side, -side, side],
                },
                PositionalVertex {
                    position: [side, -side, side],
                },
                PositionalVertex {
                    position: [side, side, side],
                },
                PositionalVertex {
                    position: [-side, side, side],
                },
                // Right
                PositionalVertex {
                    position: [side, -side, side],
                },
                PositionalVertex {
                    position: [side, -side, -side],
                },
                PositionalVertex {
                    position: [side, side, -side],
                },
                PositionalVertex {
                    position: [side, side, side],
                },
                // Back
                PositionalVertex {
                    position: [-side, -side, -side],
                },
                PositionalVertex {
                    position: [-side, side, -side],
                },
                PositionalVertex {
                    position: [side, side, -side],
                },
                PositionalVertex {
                    position: [side, -side, -side],
                },
                // Left
                PositionalVertex {
                    position: [-side, -side, side],
                },
                PositionalVertex {
                    position: [-side, side, side],
                },
                PositionalVertex {
                    position: [-side, side, -side],
                },
                PositionalVertex {
                    position: [-side, -side, -side],
                },
                // Bottom
                PositionalVertex {
                    position: [-side, -side, side],
                },
                PositionalVertex {
                    position: [-side, -side, -side],
                },
                PositionalVertex {
                    position: [side, -side, -side],
                },
                PositionalVertex {
                    position: [side, -side, side],
                },
                // Top
                PositionalVertex {
                    position: [-side, side, side],
                },
                PositionalVertex {
                    position: [side, side, side],
                },
                PositionalVertex {
                    position: [side, side, -side],
                },
                PositionalVertex {
                    position: [-side, side, -side],
                },
            ],
        )
        .unwrap()
    }

    fn create_index_buffer(display: &Display) -> IndexBuffer<u16> {
        glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &[
                // Front
                0u16, 2, 1, 0, 3, 2, // Right
                4, 6, 5, 4, 7, 6, // Back
                8, 10, 9, 8, 11, 10, // Left
                12, 14, 13, 12, 15, 14, // Bottom
                16, 18, 17, 16, 19, 18, // Top
                20, 22, 21, 20, 23, 22,
            ],
        )
        .unwrap()
    }
}
