use cgmath::{Matrix4, Point3, Vector3};
use glium::Surface;

use crate::common::ToArray;
use crate::containers::container::ObjectContainer;
use crate::objects::{
    simple_objects::{Cube, SimpleLightCube},
};
use crate::shaders::{common::Material, shader::ShaderObject, simple_shaders::CubeShader};

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
    pub cube: &'a glium::Program,
    pub light_cube: &'b glium::Program,
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
                        self.cubes.push(Cube::new(
                            Point3::new(x as f32, y as f32, z as f32),
                            Material {
                                ambient: Vector3::new(1.0, 0.5, 0.31),
                                diffuse: Vector3::new(1.0, 0.5, 0.31),
                                specular: Vector3::new(0.5, 0.5, 0.5),
                                shininess: 32.0,
                            },
                        ))
                    }
                }
            }
        }
    }

    pub fn draw_cubes(
        &self,
        target: &mut glium::Frame,
        program: &glium::Program,
        params: &glium::DrawParameters,
        projection_view: &Matrix4<f32>,
        camera_pos: Point3<f32>,
    ) {
        for cube in self.cubes.iter() {
            let matrix = projection_view * cube.object.model_matrix;

            let material_diffuse: [f32; 3] = cube.material.diffuse.into();
            let material_specular: [f32; 3] = cube.material.specular.into();

            let light_position: [f32; 3] = self.light_cubes[0].object.position.into();
            let light_diffuse: [f32; 3] = self.light_cubes[0].light.diffuse.into();
            let light_specular: [f32; 3] = self.light_cubes[0].light.specular.into();

            let ambient_color = {
                let mut temp = [0.0f32; 3];
                for i in 0..3 {
                    temp[i] = cube.material.ambient[i] * self.light_cubes[0].light.ambient[i]
                }
                temp
            };
            let view_position: [f32; 3] = camera_pos.into();

            let uniforms = uniform! {
                matrix: matrix.to_array(),
                model: cube.object.model_matrix.to_array(),

                tex: &self.shader.texture,

                material_diffuse: material_diffuse,
                material_specular: material_specular,
                material_shininess: cube.material.shininess,

                light_pos: light_position,
                light_diffuse: light_diffuse,
                light_specular: light_specular,

                ambient_color: ambient_color,
                view_pos: view_position
            };

            // draw cube
            target
                .draw(
                    &self.shader.vertex_buffer,
                    &self.shader.index_buffer,
                    program,
                    &uniforms,
                    params,
                )
                .unwrap();
        }
    }

    pub fn draw_light_cubes(
        &self,
        target: &mut glium::Frame,
        program: &glium::Program,
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
                program,
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
