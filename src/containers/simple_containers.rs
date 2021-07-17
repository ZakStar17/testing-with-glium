use cgmath::{Matrix4, Point3};
use glium::Surface;

use crate::common::ToArray;
use crate::containers::container::ObjectContainer;
use crate::objects::simple_objects::{Cube, SimpleLightCube};
use crate::shaders::{programs, shader::ShaderObject, simple_shaders::CubeShader};

pub struct CubeContainer {
    shader: CubeShader,
    pub cubes: Vec<Cube>,
    pub light_cubes: Vec<SimpleLightCube>,
}

pub struct CubeContainerDrawData<'a> {
    pub projection_view: &'a Matrix4<f32>,
    pub camera_pos: Point3<f32>,
}

pub struct CubeContainerPrograms<'a, 'b> {
    pub cube: &'a programs::SimpleTexturedObjectProgram,
    pub light_cube: &'b programs::SimpleLightObjectProgram,
}

impl CubeContainer {
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
        target: &mut glium::Frame,
        program: &programs::SimpleTexturedObjectProgram,
        params: &glium::DrawParameters,
        projection_view: &Matrix4<f32>,
        camera_pos: Point3<f32>,
    ) {
        for cube in self.cubes.iter() {
            let matrix = projection_view * cube.object.model_matrix;

            let material = &self.shader.material;
            let light = &self.light_cubes[0].light;

            let light_position: [f32; 3] = self.light_cubes[0].object.position.into();
            let light_ambient: [f32; 3] = light.ambient.into();
            let light_diffuse: [f32; 3] = light.diffuse.into();
            let light_specular: [f32; 3] = light.specular.into();

            let view_position: [f32; 3] = camera_pos.into();

            let uniforms = uniform! {
                matrix: matrix.to_array(),
                model: cube.object.model_matrix.to_array(),

                material_diffuse: &material.diffuse,
                material_specular: &material.specular,
                material_shininess: material.shininess,

                light_pos: light_position,
                light_ambient: light_ambient,
                light_diffuse: light_diffuse,
                light_specular: light_specular,

                view_pos: view_position
            };

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
        target: &mut glium::Frame,
        program: &programs::SimpleLightObjectProgram,
        params: &glium::DrawParameters,
        projection_view: &Matrix4<f32>,
    ) {
        // todo: make it work with multiple lights
        let matrix = projection_view * self.light_cubes[0].object.model_matrix;
        let color: [f32; 3] = self.light_cubes[0].light.diffuse.into();

        let uniforms = uniform! {
            matrix: matrix.to_array(),
            color: color
        };

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

impl<'a> ObjectContainer<CubeContainerPrograms<'_, '_>, CubeContainerDrawData<'_>>
    for CubeContainer
{
    fn new(display: &glium::Display) -> Self {
        CubeContainer {
            shader: CubeShader::new(&display),
            cubes: Vec::new(),
            light_cubes: Vec::new(),
        }
    }

    fn draw(
        &self,
        target: &mut glium::Frame,
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
        );
        self.draw_light_cubes(target, programs.light_cube, params, data.projection_view);
    }
}
