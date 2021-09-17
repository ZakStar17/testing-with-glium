use crate::shaders::kakyoin_shader::KakyoinShader;
use cgmath::{Matrix4, Point3, Vector3};
use glium::Surface;

use crate::containers::container::ObjectContainer;
use crate::objects::kakyoin::Kakyoin;
use crate::shaders::{
    common::{DirectionalLight, PointLight, SpotLight},
    programs,
};

pub struct KakyoinContainer {
    pub shader: KakyoinShader,
    pub kakyoins: Vec<Kakyoin>,
}

pub struct KakyoinContainerDrawData<'a, 'b, 'c, 'd> {
    pub projection_view: &'a Matrix4<f32>,
    pub camera_pos: Point3<f32>,
    pub spot_light: &'b SpotLight,
    pub point_lights: &'c [&'d PointLight; 4],
    pub t: f32,
}

pub struct KakyoinContainerPrograms<'a> {
    pub kakyoin: &'a programs::SimpleTexturedObjectProgram,
}

impl KakyoinContainer {
    pub fn new(display: &glium::Display) -> Self {
        Self {
            shader: KakyoinShader::new(display),
            kakyoins: Vec::new(),
        }
    }

    pub fn draw_kakyoins(
        &self,
        target: &mut glium::framebuffer::SimpleFrameBuffer,
        program: &programs::SimpleTexturedObjectProgram,
        params: &glium::DrawParameters,
        projection_view: &Matrix4<f32>,
        camera_pos: Point3<f32>,
        spot_light: &SpotLight,
        lights: &[&PointLight; 4],
        t: f32,
    ) {
        for kakyoin in self.kakyoins.iter() {
            let matrix = projection_view * kakyoin.object.model_matrix;

            let directional_light = {
                let ambient = t / 3.0;
                let diffuse = t;
                let specular = t * 0.4 + 0.4;
                DirectionalLight {
                    ambient: Vector3::new(ambient, ambient, ambient),
                    diffuse: Vector3::new(diffuse, diffuse, diffuse),
                    specular: Vector3::new(specular, specular, specular),
                    direction: Vector3::new(-0.2, -1.0, -0.3),
                }
            };

            let uniforms = programs::SimpleTexturedObjectProgram::get_uniforms(
                &matrix,
                &kakyoin.object.model_matrix,
                &self.shader.material,
                &directional_light,
                spot_light,
                &lights,
                &camera_pos,
            );

            target
                .draw(
                    &self.shader.vertex_buffer,
                    &self.shader.index_buffer,
                    &program.0,
                    &uniforms,
                    params,
                )
                .unwrap();
        }
    }
}

impl<'a> ObjectContainer<KakyoinContainerPrograms<'_>, KakyoinContainerDrawData<'_, '_, '_, '_>>
    for KakyoinContainer
{
    fn draw(
        &self,
        target: &mut glium::framebuffer::SimpleFrameBuffer,
        programs: KakyoinContainerPrograms,
        params: &glium::DrawParameters,
        data: KakyoinContainerDrawData,
    ) {
        self.draw_kakyoins(
            target,
            programs.kakyoin,
            params,
            data.projection_view,
            data.camera_pos,
            data.spot_light,
            data.point_lights,
            data.t,
        );
    }
}
