#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate image;

use std::f32::consts::PI;

use cgmath::{InnerSpace, Matrix4, PerspectiveFov, Point3, Rad, Vector3};
use glium::{glutin, Surface};

mod common;
mod cube;

use common::ToArray;
use cube::{Cube, CubeConfig};

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
    let event_loop = glutin::event_loop::EventLoop::new();

    let display = {
        let wb = glutin::window::WindowBuilder::new()
            .with_title("Testing with glium")
            .with_inner_size(glutin::dpi::LogicalSize::new(768.0, 768.0));

        let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
        glium::Display::new(wb, cb, &event_loop).unwrap()
    };

    {
        // window configuration
        let gl_window = display.gl_window();
        let window = gl_window.window();
        window.set_cursor_grab(true).unwrap();
        window.set_cursor_visible(false);
    }

    let row_cube_count: usize = 7; // odd number
    let cube_config = CubeConfig::new(&display);
    let mut cubes: Vec<Cube> = Vec::with_capacity(row_cube_count.pow(3));
    let a = ((row_cube_count - 1) * 2) as i32;
    for x in (-a..=a).step_by(4) {
        for y in (-a..=a).step_by(4) {
            for z in (-a..=a).step_by(4) {
                cubes.push(Cube::new(Point3::new(x as f32, y as f32, z as f32)))
            }
        }
    }

    let mut camera = Camera {
        position: Point3::new(0.0, 0.0, 3.0),
        front: Vector3::new(0.0, 0.0, -1.0),
        up: Vector3::new(0.0, 1.0, 0.0),
        speed: 4.0,
    };

    let mut yaw: f32 = 0.0;
    let mut pitch: f32 = 0.0;
    let mut mouse_delta_x: f32 = 0.0;
    let mut mouse_delta_y: f32 = 0.0;
    let sensitivity: f32 = 0.005;
    let mut mouse_in_window = false;
    let mut mouse_centered = false;
    let mut fov: f32 = 0.8;

    let mut pressed_keys = [false; 4];

    let mut last_frame_time = std::time::Instant::now();
    event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                glutin::event::WindowEvent::KeyboardInput {
                    device_id: _,
                    input,
                    is_synthetic: _,
                } => {
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
                #[allow(deprecated)]
                // todo: maybe there is another way to hide the unused deprecated variable
                glutin::event::WindowEvent::CursorMoved {
                    device_id: _,
                    position,
                    modifiers: _,
                } => {
                    // capture mouse movement
                    // todo: the documentation says to not use this event for the game purpose, do it another way

                    let gl_window = display.gl_window();
                    let window = gl_window.window();

                    let size = window.inner_size();
                    let middle = (size.width as f64 / 2.0, size.height as f64 / 2.0);

                    if mouse_in_window && mouse_centered {
                        mouse_delta_x = (position.x - middle.0) as f32 * sensitivity;
                        mouse_delta_y = (middle.1 - position.y) as f32 * sensitivity;
                    }

                    window
                        .set_cursor_position(glutin::dpi::Position::new(
                            glutin::dpi::PhysicalPosition::<f64>::new(middle.0, middle.1),
                        ))
                        .unwrap();

                    mouse_centered = true;
                }
                glutin::event::WindowEvent::CursorEntered { device_id: _ } => {
                    mouse_in_window = true;
                }
                glutin::event::WindowEvent::CursorLeft { device_id: _ } => {
                    mouse_in_window = false;
                    mouse_centered = false;
                }
                #[allow(deprecated)]
                glutin::event::WindowEvent::MouseWheel {device_id: _, delta, phase: _, modifiers: _} => {
                    if let glium::glutin::event::MouseScrollDelta::LineDelta(_, y) = delta {
                        fov -= y * 0.07;
                        if fov > PI / 1.1 {
                            fov = PI / 1.1;
                        }
                        else if fov < PI / 15.0 {
                            fov = PI / 15.0;
                        }
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

        yaw += mouse_delta_x;
        pitch += mouse_delta_y;

        if pitch > 1.5 {
            pitch = 1.5;
        } else if pitch < -1.5 {
            pitch = -1.5;
        }

        let direction: Vector3<f32> = {
            let x = yaw.cos() * pitch.cos();
            let y = pitch.sin();
            let z = yaw.sin() * pitch.cos();
            Vector3::new(x, y, z)
        };

        camera.front = direction;

        // todo: this looks horrible
        let delta_camera_speed = camera.speed * delta_time.as_secs_f32();
        if pressed_keys[0] {
            // w
            camera.position += camera.front * delta_camera_speed;
        }
        if pressed_keys[1] {
            // a
            camera.position -= camera.front.cross(camera.up).normalize() * delta_camera_speed;
        }
        if pressed_keys[2] {
            // s
            camera.position -= camera.front * delta_camera_speed;
        }
        if pressed_keys[3] {
            // d
            camera.position += camera.front.cross(camera.up).normalize() * delta_camera_speed;
        }

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let projection = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = width as f32 / height as f32;

            PerspectiveFov {
                fovy: Rad(fov),
                aspect: aspect_ratio,
                far: 100.0,
                near: 0.1,
            }
            .to_array()
        };
        let view = camera.get_view_matrix().to_array();

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        for cube in cubes.iter() {
            let uniforms = uniform! {
                projection: projection,
                view: view,
                model: cube.model_matrix,
                tex: &cube_config.texture,
            };
            // draw cube
            target
                .draw(
                    &cube_config.vertex_buffer,
                    &cube_config.index_buffer,
                    &cube_config.program,
                    &uniforms,
                    &params,
                )
                .unwrap();
        }
        target.finish().unwrap();

        last_frame_time = current_frame_time;
        mouse_delta_x = 0.0;
        mouse_delta_y = 0.0
    });
}
