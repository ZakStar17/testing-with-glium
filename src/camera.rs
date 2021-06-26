use cgmath::{InnerSpace, Matrix4, PerspectiveFov, Point3, Rad, Vector3};

use std::f32::consts::PI;

const HALF_PI: f32 = PI / 2.0;

pub struct Camera {
    pub position: Point3<f32>,
    pub front: Vector3<f32>,
    pub up: Vector3<f32>,
    pub yaw: f32,
    pub pitch: f32,
    pub speed: f32,
    pub sensitivity: f32,
    pub fov: f32,
}

impl Camera {
    pub fn new(position: Point3<f32>) -> Camera {
        Camera {
            position,
            front: Vector3::new(0.0, 0.0, -1.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
            speed: 4.0,
            sensitivity: 0.005,
            fov: 0.8,
        }
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(self.position, self.position + self.front, self.up)
    }

    pub fn get_projection_matrix(&self, aspect_ratio: f32) -> Matrix4<f32> {
        PerspectiveFov {
            fovy: Rad(self.fov),
            aspect: aspect_ratio,
            far: 100.0,
            near: 0.1,
        }
        .into()
    }

    pub fn handle_zoom(&mut self, ammount: f32) {
        self.fov -= ammount * 0.07;
        if self.fov > PI / 1.1 {
            self.fov = PI / 1.1;
        } else if self.fov < PI / 15.0 {
            self.fov = PI / 15.0;
        }
    }

    pub fn handle_mouse_movement(&mut self, delta_x: f32, delta_y: f32) {
        self.yaw += delta_x * self.sensitivity;
        self.pitch += delta_y * self.sensitivity;

        if self.pitch > HALF_PI - 0.1 {
            self.pitch = HALF_PI - 0.1;
        } else if self.pitch < -HALF_PI + 0.1 {
            self.pitch = -HALF_PI + 0.1;
        }

        self.front = {
            let pitch_cos = self.pitch.cos();
            let x = self.yaw.cos() * pitch_cos;
            let y = self.pitch.sin();
            let z = self.yaw.sin() * pitch_cos;
            Vector3::new(x, y, z)
        };
    }

    pub fn handle_keys(&mut self, keys_pressed: [bool; 4], delta_time: std::time::Duration) {
        // todo: handle keys better
        let delta_speed = self.speed * delta_time.as_secs_f32();
        if keys_pressed[0] {
            // w
            self.position += self.front * delta_speed;
        }
        if keys_pressed[1] {
            // a
            self.position -= self.front.cross(self.up).normalize() * delta_speed;
        }
        if keys_pressed[2] {
            // s
            self.position -= self.front * delta_speed;
        }
        if keys_pressed[3] {
            // d
            self.position += self.front.cross(self.up).normalize() * delta_speed;
        }
    }
}
