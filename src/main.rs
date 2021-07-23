#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate image;

use cgmath::{Euler, Point3, Rad, Vector3};
use glium::{glutin, Surface};
use glutin::event::WindowEvent;

mod camera;
mod common;
mod containers;
mod objects;
mod shaders;

use camera::Camera;
use containers::{
    container::ObjectContainer,
    simple_containers::{CubeContainer, CubeContainerDrawData, CubeContainerPrograms},
};
use objects::simple_objects::SimpleLightCube;
use shaders::{
    common::{PointLight, SpotLight},
    programs,
    programs::PostProcessingEffects,
};

struct Mouse {
    delta_x: f32,
    delta_y: f32,
    in_window: bool,
    centered: bool,
}

struct Programs {
    textured_object: programs::SimpleTexturedObjectProgram,
    light_object: programs::SimpleLightObjectProgram,
    main_framebuffer: programs::MainFramebufferProgram,
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

    let mut cube_container = {
        let positions = [
            Point3::new(1.0, 2.0, 3.0),
            Point3::new(4.0, 2.0, 7.0),
            Point3::new(-2.0, 2.0, 0.0),
            Point3::new(-5.1, 2.0, -3.1),
        ];

        let light_cubes = [
            SimpleLightCube::new(
                Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)),
                0.2,
                PointLight {
                    position: positions[0],
                    ambient: Vector3::new(0.002, 0.002, 0.002),
                    diffuse: Vector3::new(1.0, 1.0, 1.0),
                    specular: Vector3::new(1.0, 1.0, 1.0),
                    constant: 1.0,
                    linear: 0.045,
                    quadratic: 0.0075,
                },
            ),
            SimpleLightCube::new(
                Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)),
                0.2,
                PointLight {
                    position: positions[1],

                    ambient: Vector3::new(0.002, 0.002, 0.002),
                    diffuse: Vector3::new(1.0, 1.0, 1.0),
                    specular: Vector3::new(1.0, 1.0, 1.0),

                    constant: 1.0,
                    linear: 0.045,
                    quadratic: 0.0075,
                },
            ),
            SimpleLightCube::new(
                Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)),
                0.2,
                PointLight {
                    position: positions[2],

                    ambient: Vector3::new(0.002, 0.002, 0.002),
                    diffuse: Vector3::new(1.0, 1.0, 1.0),
                    specular: Vector3::new(1.0, 1.0, 1.0),

                    constant: 1.0,
                    linear: 0.045,
                    quadratic: 0.0075,
                },
            ),
            SimpleLightCube::new(
                Euler::new(Rad(0.0), Rad(0.0), Rad(0.0)),
                0.2,
                PointLight {
                    position: positions[3],

                    ambient: Vector3::new(0.002, 0.002, 0.002),
                    diffuse: Vector3::new(1.0, 1.0, 1.0),
                    specular: Vector3::new(1.0, 1.0, 1.0),

                    constant: 1.0,
                    linear: 0.045,
                    quadratic: 0.0075,
                },
            ),
        ];
        CubeContainer::new(&display, light_cubes)
    };
    cube_container.generate_cubes();

    let main_framebuffer_shader =
        crate::shaders::main_framebuffer_shader::MainFramebufferShader::new(&display);

    let post_processing_effects = [
        PostProcessingEffects::NoPostProcessing,
        PostProcessingEffects::Inversed,
        PostProcessingEffects::GrayScale,
        PostProcessingEffects::DeepFried,
        PostProcessingEffects::Blur,
        PostProcessingEffects::Edged,
    ];
    let mut selected_post_processing_effect_i = 0;

    let mut programs = Programs {
        textured_object: programs::SimpleTexturedObjectProgram::new(&display),
        light_object: programs::SimpleLightObjectProgram::new(&display),
        main_framebuffer: programs::MainFramebufferProgram::new(
            &display,
            &post_processing_effects[selected_post_processing_effect_i],
        ),
    };

    let mut camera = Camera::new(Point3::new(0.0, 0.0, 3.0));

    let mut spot_light = SpotLight {
        position: camera.position,
        direction: camera.front,
        cut_off: 0.97629600712,
        outer_cut_off: 0.953716950748,

        ambient: Vector3::new(0.02, 0.02, 0.02),
        diffuse: Vector3::new(1.0, 1.0, 1.0),
        specular: Vector3::new(1.0, 1.0, 1.0),
    };

    let mut projection_matrix = camera.get_projection_matrix(get_aspect_ratio(&display));

    let mut mouse = Mouse {
        delta_x: 0.0,
        delta_y: 0.0,
        in_window: false,
        centered: false,
    };

    let mut pressed_keys = [false; 4];

    let mut t: f32 = 0.01;
    let mut flashlight = true;

    let mut last_frame_time = std::time::Instant::now();
    event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                WindowEvent::KeyboardInput {
                    device_id: _,
                    input,
                    is_synthetic: _,
                } => {
                    let was_pressed = input.state == glutin::event::ElementState::Pressed;

                    match input.scancode {
                        1 => {
                            *control_flow = glutin::event_loop::ControlFlow::Exit;
                            return;
                        }
                        17 => {
                            // w
                            pressed_keys[0] = was_pressed;
                        }
                        30 => {
                            // a
                            pressed_keys[1] = was_pressed;
                        }
                        31 => {
                            // s
                            pressed_keys[2] = was_pressed;
                        }
                        32 => {
                            // d
                            pressed_keys[3] = was_pressed;
                        }
                        20 => {
                            // t
                            t += 0.01;
                        }
                        21 => {
                            // y
                            if t > 0.0 {
                                t -= 0.01;
                            }
                        }
                        44 => {
                            // z
                            if !was_pressed {
                                if selected_post_processing_effect_i == 0 {
                                    selected_post_processing_effect_i =
                                        post_processing_effects.len() - 1;
                                } else {
                                    selected_post_processing_effect_i -= 1
                                }
                                programs.main_framebuffer = programs::MainFramebufferProgram::new(
                                    &display,
                                    &post_processing_effects[selected_post_processing_effect_i],
                                );
                            }
                        }
                        45 => {
                            // x
                            if !was_pressed {
                                if selected_post_processing_effect_i
                                    == post_processing_effects.len() - 1
                                {
                                    selected_post_processing_effect_i = 0;
                                } else {
                                    selected_post_processing_effect_i += 1
                                }
                                programs.main_framebuffer = programs::MainFramebufferProgram::new(
                                    &display,
                                    &post_processing_effects[selected_post_processing_effect_i],
                                );
                            }
                        }
                        38 => {
                            // l
                            if !was_pressed {
                                flashlight = !flashlight;

                                // todo: repeated code
                                if flashlight {
                                    spot_light.ambient = Vector3::new(0.002, 0.002, 0.002);
                                    spot_light.diffuse = Vector3::new(0.5, 0.5, 0.5);
                                    spot_light.specular = Vector3::new(0.5, 0.5, 0.5);
                                } else {
                                    spot_light.ambient = Vector3::new(0.0, 0.0, 0.0);
                                    spot_light.diffuse = Vector3::new(0.0, 0.0, 0.0);
                                    spot_light.specular = Vector3::new(0.0, 0.0, 0.0);
                                }
                            }
                        }
                        x => {
                            println!("{}", x);
                        }
                    }
                }
                #[allow(deprecated)]
                // todo: maybe there is another way to hide the unused deprecated variable
                WindowEvent::CursorMoved {
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

                    if mouse.in_window && mouse.centered {
                        mouse.delta_x = (position.x - middle.0) as f32;
                        mouse.delta_y = (middle.1 - position.y) as f32;
                    }

                    window
                        .set_cursor_position(glutin::dpi::Position::new(
                            glutin::dpi::PhysicalPosition::<f64>::new(middle.0, middle.1),
                        ))
                        .unwrap();

                    mouse.centered = true;
                }
                WindowEvent::CursorEntered { device_id: _ } => {
                    mouse.in_window = true;
                }
                WindowEvent::CursorLeft { device_id: _ } => {
                    mouse.in_window = false;
                    mouse.centered = false;
                }
                #[allow(deprecated)]
                WindowEvent::MouseWheel {
                    device_id: _,
                    delta,
                    phase: _,
                    modifiers: _,
                } => {
                    if let glium::glutin::event::MouseScrollDelta::LineDelta(_, y) = delta {
                        camera.handle_zoom(y);
                        // update projection_matrix because of fov changes
                        projection_matrix = camera.get_projection_matrix(get_aspect_ratio(&display))
                    }
                }
                WindowEvent::Resized(new_size) => {
                    projection_matrix =
                        camera.get_projection_matrix(new_size.width as f32 / new_size.height as f32)
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

        // max framerate = 60fps
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        let current_frame_time = std::time::Instant::now();
        let delta_time = current_frame_time - last_frame_time;

        camera.handle_mouse_movement(mouse.delta_x, mouse.delta_y);
        camera.handle_keys(pressed_keys, delta_time);

        // update spot_light
        spot_light.position = camera.position;
        spot_light.direction = camera.front;

        let mut target = display.draw();
        let size = target.get_dimensions();

        let render_texture = glium::Texture2d::empty_with_format(
            &display,
            glium::texture::UncompressedFloatFormat::U8U8U8U8,
            glium::texture::MipmapsOption::NoMipmap,
            size.0,
            size.1,
        )
        .unwrap();

        let depth_buffer = glium::framebuffer::DepthRenderBuffer::new(
            &display,
            glium::texture::DepthFormat::I24,
            size.0,
            size.1,
        )
        .unwrap();

        let mut framebuffer = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(
            &display,
            &render_texture,
            &depth_buffer,
        )
        .unwrap();

        framebuffer.clear_color_srgb_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        // target.clear_depth(1.0);

        let view_matrix = camera.get_view_matrix();

        let projection_view = projection_matrix * view_matrix;

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };

        cube_container.draw(
            &mut framebuffer,
            CubeContainerPrograms {
                cube: &programs.textured_object,
                light_cube: &programs.light_object,
            },
            &params,
            CubeContainerDrawData {
                projection_view: &projection_view,
                camera_pos: camera.position,
                spot_light: &spot_light,
                t: t,
            },
        );

        // draw framebuffer to target (with post processing effects)
        target.clear_depth(1.0);
        let uniforms = programs::MainFramebufferProgram::get_uniforms(&render_texture);
        target
            .draw(
                &main_framebuffer_shader.vertex_buffer,
                &main_framebuffer_shader.index_buffer,
                &programs.main_framebuffer.0,
                &uniforms,
                &params,
            )
            .unwrap();
        target.finish().unwrap();

        last_frame_time = current_frame_time;
        mouse.delta_x = 0.0;
        mouse.delta_y = 0.0
    });
}

fn get_aspect_ratio(display: &glium::Display) -> f32 {
    let size = display.gl_window().window().inner_size();
    size.width as f32 / size.height as f32
}
