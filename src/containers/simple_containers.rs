use cgmath::{Matrix4, Point3, Vector3};
use glium::Surface;

use crate::containers::container::ObjectContainer;
use crate::objects::simple_objects::{Cube, SimpleLightCube};
use crate::shaders::{
    common::{DirectionalLight, SpotLight},
    programs,
    simple_shaders::CubeShader,
};

pub struct CubeContainer {
    pub shader: CubeShader,
    pub cubes: Vec<Cube>,
    pub light_cubes: [SimpleLightCube; 4],
}

pub struct CubeContainerDrawData<'a, 'b> {
    pub projection_view: &'a Matrix4<f32>,
    pub camera_pos: Point3<f32>,
    pub spot_light: &'b SpotLight,
    pub t: f32,
}

pub struct CubeContainerPrograms<'a, 'b> {
    pub cube: &'a programs::SimpleTexturedObjectProgram,
    pub light_cube: &'b programs::SimpleLightObjectProgram,
}

impl CubeContainer {
    pub fn new(display: &glium::Display, lights: [SimpleLightCube; 4]) -> Self {
        CubeContainer {
            shader: CubeShader::new(&display),
            cubes: Vec::new(),
            light_cubes: lights,
        }
    }

    pub fn generate_cubes(&mut self) {
        let row_cube_count: usize = 3; // odd number
        self.cubes = Vec::with_capacity(row_cube_count.pow(3));
        {
            let a = ((row_cube_count - 1) * 2) as i32;
            for x in (-a..=a).step_by(4) {
                for y in (-a..=a).step_by(4) {
                    for z in (-a..=a).step_by(4) {
                        self.cubes
                            .push(Cube::new(Point3::new(x as f32, y as f32, z as f32)))
                    }
                }
            }
        }
    }

    pub fn draw_cubes(
        &self,
        target: &mut glium::framebuffer::SimpleFrameBuffer,
        program: &programs::SimpleTexturedObjectProgram,
        params: &glium::DrawParameters,
        projection_view: &Matrix4<f32>,
        camera_pos: Point3<f32>,
        spot_light: &SpotLight,
        t: f32,
    ) {
        for cube in self.cubes.iter() {
            let matrix = projection_view * cube.object.model_matrix;

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

            let lights = [
                &self.light_cubes[0].light,
                &self.light_cubes[1].light,
                &self.light_cubes[2].light,
                &self.light_cubes[3].light,
            ];

            let uniforms = programs::SimpleTexturedObjectProgram::get_uniforms(
                &matrix,
                &cube.object.model_matrix,
                &self.shader.material,
                &directional_light,
                spot_light,
                &lights,
                &camera_pos,
            );

            // draw cube
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

    pub fn draw_light_cubes(
        &self,
        target: &mut glium::framebuffer::SimpleFrameBuffer,
        program: &programs::SimpleLightObjectProgram,
        params: &glium::DrawParameters,
        projection_view: &Matrix4<f32>,
    ) {
        for light_cube in self.light_cubes.iter() {
            let matrix = projection_view * light_cube.object.model_matrix;

            let uniforms = programs::SimpleLightObjectProgram::get_uniforms(
                &matrix,
                &light_cube.light.diffuse,
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

impl<'a> ObjectContainer<CubeContainerPrograms<'_, '_>, CubeContainerDrawData<'_, '_>>
    for CubeContainer
{
    fn draw(
        &self,
        target: &mut glium::framebuffer::SimpleFrameBuffer,
        programs: CubeContainerPrograms,
        params: &glium::DrawParameters,
        data: CubeContainerDrawData,
    ) {
        self.draw_cubes(
            target,
            programs.cube,
            params,
            data.projection_view,
            data.camera_pos,
            data.spot_light,
            data.t,
        );
        self.draw_light_cubes(target, programs.light_cube, params, data.projection_view);
    }
}
