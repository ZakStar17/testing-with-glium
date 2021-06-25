#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate image;

use std::io::Cursor;

use cgmath::{Euler, Matrix4, PerspectiveFov, Rad, Vector3, InnerSpace, Point3};
use glium::{glutin, Surface};

pub trait ToArr {
    type Output;
    fn to_arr(&self) -> Self::Output;
}

impl ToArr for Matrix4<f32> {
    type Output = [[f32; 4]; 4];
    fn to_arr(&self) -> Self::Output {
        (*self).into()
    }
}

impl ToArr for PerspectiveFov<f32> {
    type Output = [[f32; 4]; 4];
    fn to_arr(&self) -> Self::Output {
        let matrix: Matrix4<f32> = (*self).into();
        matrix.to_arr()
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

struct Camera {
    position: Point3<f32>,
    front: Vector3<f32>,
    up: Vector3<f32>,
    speed: f32,
}

impl Camera {
    fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(self.position, self.position + self.front, self.up)
    }
}

fn main() {
    implement_vertex!(Vertex, position, tex_coords);

    let event_loop = glutin::event_loop::EventLoop::new();

    let display = {
        let wb = glutin::window::WindowBuilder::new()
            .with_title("Testing with glium")
            .with_inner_size(glutin::dpi::LogicalSize::new(768.0, 768.0));
        let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
        glium::Display::new(wb, cb, &event_loop).unwrap()
    };

    let texture = {
        let image = image::load(
            Cursor::new(&include_bytes!("./wall.png")),
            image::ImageFormat::Png,
        )
        .unwrap()
        .to_rgba8();

        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        glium::texture::Texture2d::new(&display, image).unwrap()
    };

    let cube_program = {
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
                // color = texture(tex, v_tex_coords);
            }
        "#;

        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap()
    };

    let cube_vertex_buffer = {
        let shape = get_cube_shape();
        glium::VertexBuffer::new(&display, &shape).unwrap()
    };

    let cube_indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &get_cube_indices(),
    )
    .unwrap();

    let mut camera = Camera {
        position: Point3::new(0.0, 0.0, 3.0),
        front: Vector3::new(0.0, 0.0, -1.0),
        up: Vector3::new(0.0, 1.0, 0.0),
        speed: 3.0
    };

    let mut pressed_keys = [false; 4];

    let mut last_frame_time = std::time::Instant::now();
    
    event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glutin::event::WindowEvent::KeyboardInput {device_id: _, input, is_synthetic: _} => {
                    let was_pressed = input.state == glutin::event::ElementState::Pressed;

                    match input.scancode {
                        17 => {
                            // w
                            pressed_keys[0] = was_pressed
                        }
                        30 => {
                            // a
                            pressed_keys[1] = was_pressed
                        }
                        31 => {
                            // s
                            pressed_keys[2] = was_pressed
                        }
                        32 => {
                            // d
                            pressed_keys[3] = was_pressed
                        }
                        _ => {}
                    }
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        let current_frame_time = std::time::Instant::now();
        let delta_time = current_frame_time - last_frame_time;

        // todo: this looks horrible
        let delta_camera_speed = camera.speed * delta_time.as_secs_f32();
        if pressed_keys[0] {  // w
            camera.position += camera.front * delta_camera_speed;
        }
        if pressed_keys[1] {  // a
            camera.position -= camera.front.cross(camera.up).normalize() * delta_camera_speed;
        }
        if pressed_keys[2] {  // s
            camera.position -= camera.front * delta_camera_speed;
        }
        if pressed_keys[3] {  // d
            camera.position += camera.front.cross(camera.up).normalize() * delta_camera_speed;
        }

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let projection = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = width as f32 / height as f32;

            PerspectiveFov {
                fovy: Rad(0.8539816),
                aspect: aspect_ratio,
                far: 100.0,
                near: 0.1,
            }
            .to_arr()
        };
        let view = camera.get_view_matrix().to_arr();
        let model = {
            let translation_matrix = Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0));
            let rotation_matrix = Matrix4::from(Euler {
                x: Rad(0.0),
                y: Rad(0.0),
                z: Rad(0.0),
            });
            let scale_matrix = Matrix4::from_scale(1.5);

            (translation_matrix * rotation_matrix * scale_matrix).to_arr()
        };

        let uniforms = uniform! {
            projection: projection,
            view: view,
            model: model,
            tex: &texture,
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        // draw cube
        target
            .draw(&cube_vertex_buffer, &cube_indices, &cube_program, &uniforms, &params)
            .unwrap();
        target.finish().unwrap();

        last_frame_time = current_frame_time;
    });
}

fn get_cube_shape() -> Vec<Vertex> {
    vec![
        Vertex {
            position: [-0.5, 0.5, -0.5],
            tex_coords: [0.0, 1.0],
        },
        Vertex {
            position: [-0.5, -0.5, -0.5],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: [0.5, -0.5, -0.5],
            tex_coords: [1.0, 0.0],
        },
        Vertex {
            position: [0.5, 0.5, -0.5],
            tex_coords: [1.0, 1.0],
        },
        Vertex {
            position: [-0.5, 0.5, 0.5],
            tex_coords: [0.0, 1.0],
        },
        Vertex {
            position: [-0.5, -0.5, 0.5],
            tex_coords: [0.0, 0.0],
        },
        Vertex {
            position: [0.5, -0.5, 0.5],
            tex_coords: [1.0, 0.0],
        },
        Vertex {
            position: [0.5, 0.5, 0.5],
            tex_coords: [1.0, 1.0],
        },
    ]
}

fn get_cube_indices() -> [u8; 36] {
    [
        0, 2, 1, 0, 2, 3, 0, 7, 3, 0, 7, 4, 1, 4, 0, 1, 4, 5, 2, 5, 1, 2, 5, 6, 3, 6, 2, 3, 6, 7,
        4, 6, 5, 4, 6, 7,
    ]
}
