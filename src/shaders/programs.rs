use cgmath::{Matrix4, Point3, Vector3};
use glium::{uniforms::UniformValue, Display, Program};

use crate::common::ToArray;
use crate::shaders::common::{DirectionalLight, Material, PointLight, SpotLight};

pub struct SimpleTexturedObjectProgram(pub Program);

pub struct SimpleTexturedObjectUniforms<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
    matrix: &'a Matrix4<f32>,
    model: &'b Matrix4<f32>,
    material: &'c Material,
    directional_light: &'d DirectionalLight,
    spot_light: &'e SpotLight,
    point_lights: &'f [&'f PointLight; 4],
    view: &'g Point3<f32>,
}

impl glium::uniforms::Uniforms for SimpleTexturedObjectUniforms<'_, '_, '_, '_, '_, '_, '_> {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut f: F) {
        f("matrix", UniformValue::Mat4(self.matrix.to_array()));
        f("model", UniformValue::Mat4(self.model.to_array()));

        // todo: use a macro or something to simplify this redundant code
        f(
            "material.diffuse",
            UniformValue::SrgbTexture2d(&self.material.diffuse, None),
        );
        f(
            "material.specular",
            UniformValue::SrgbTexture2d(&self.material.specular, None),
        );
        f(
            "material.shininess",
            UniformValue::Float(self.material.shininess),
        );

        f(
            "directional_light.direction",
            UniformValue::Vec3(self.directional_light.direction.into()),
        );
        f(
            "directional_light.ambient",
            UniformValue::Vec3(self.directional_light.ambient.into()),
        );
        f(
            "directional_light.diffuse",
            UniformValue::Vec3(self.directional_light.diffuse.into()),
        );
        f(
            "directional_light.specular",
            UniformValue::Vec3(self.directional_light.specular.into()),
        );

        f(
            "spot_light.position",
            UniformValue::Vec3(self.spot_light.position.into()),
        );
        f(
            "spot_light.direction",
            UniformValue::Vec3(self.spot_light.direction.into()),
        );
        f(
            "spot_light.cut_off",
            UniformValue::Float(self.spot_light.cut_off),
        );
        f(
            "spot_light.outer_cut_off",
            UniformValue::Float(self.spot_light.outer_cut_off),
        );
        f(
            "spot_light.ambient",
            UniformValue::Vec3(self.spot_light.ambient.into()),
        );
        f(
            "spot_light.specular",
            UniformValue::Vec3(self.spot_light.specular.into()),
        );
        f(
            "spot_light.diffuse",
            UniformValue::Vec3(self.spot_light.diffuse.into()),
        );

        for (i, point_light) in self.point_lights.iter().enumerate() {
            f(
                &format!("point_lights[{}].position", i),
                UniformValue::Vec3(point_light.position.into()),
            );
            f(
                &format!("point_lights[{}].ambient", i),
                UniformValue::Vec3(point_light.ambient.into()),
            );
            f(
                &format!("point_lights[{}].diffuse", i),
                UniformValue::Vec3(point_light.diffuse.into()),
            );
            f(
                &format!("point_lights[{}].specular", i),
                UniformValue::Vec3(point_light.specular.into()),
            );

            f(
                &format!("point_lights[{}].constant", i),
                UniformValue::Float(point_light.constant),
            );
            f(
                &format!("point_lights[{}].linear", i),
                UniformValue::Float(point_light.linear),
            );
            f(
                &format!("point_lights[{}].quadratic", i),
                UniformValue::Float(point_light.quadratic),
            );
        }

        f(
            "view",
            UniformValue::Vec3([self.view.x, self.view.y, self.view.z]),
        );
    }
}

impl SimpleTexturedObjectProgram {
    pub fn new(display: &Display) -> SimpleTexturedObjectProgram {
        let vertex_shader_src = r#"
            #version 330 core
    
            in vec3 position;
            in vec3 normal;
            in vec2 tex_coords;
    
            out vec2 v_tex_coords;
            out vec3 v_frag_pos;
            out vec3 v_normal;
    
            uniform mat4 matrix;
            uniform mat4 model;
    
            void main() {
                gl_Position = matrix * vec4(position, 1.0);
    
                v_tex_coords = tex_coords;
                v_normal = normal;
                v_frag_pos = vec3(model * vec4(position, 1.0));
            }
        "#;
        let fragment_shader_src = r#"
            #version 330 core
    
            in vec2 v_tex_coords;
            in vec3 v_normal;
            in vec3 v_frag_pos;
    
            out vec4 out_color;

            struct Material {
                sampler2D diffuse;
                sampler2D specular;
                float shininess;
            };
            uniform Material material;

            struct DirectionalLight {
                vec3 direction;
                vec3 ambient;
                vec3 diffuse;
                vec3 specular;
            };
            uniform DirectionalLight directional_light;

            struct SpotLight {
                vec3 position;
                vec3 direction;
                float cut_off;
                float outer_cut_off;

                vec3 ambient;
                vec3 diffuse;
                vec3 specular;
            };
            uniform SpotLight spot_light;
    
            struct PointLight {
                vec3 position;
                vec3 ambient;
                vec3 diffuse;
                vec3 specular;

                float constant;
                float linear;
                float quadratic;
            };
            #define NR_POINT_LIGHTS 4
            uniform PointLight point_lights[NR_POINT_LIGHTS];
    
            uniform vec3 view_pos;

            vec3 calculate_directional_light(DirectionalLight light, vec3 normal, vec3 view_direction, vec3 tex_diffuse, vec3 tex_specular) {
                vec3 light_dir = normalize(-light.direction);

                // diffuse shading
                float diff = max(dot(normal, light_dir), 0.0);

                // specular shading
                vec3 reflect_direction = reflect(-light_dir, normal);
                float spec = pow(max(dot(light.direction, reflect_direction), 0.0),
                                 material.shininess);

                // combine results
                vec3 ambient = light.ambient * tex_diffuse;
                vec3 diffuse = light.diffuse * diff * tex_diffuse;
                vec3 specular = light.specular * spec * tex_specular; 
                return (ambient + diffuse + specular);
            }


            vec3 calculate_spot_light(SpotLight light, vec3 normal, vec3 frag_pos, vec3 view_direction, vec3 tex_diffuse, vec3 tex_specular) {
                // diffuse
                vec3 light_direction = normalize(light.position - frag_pos);
                float diff = max(dot(normal, light_direction), 0.0);
    
                // specular
                vec3 reflect_direction = reflect(-light_direction, normal);  
                float spec = pow(max(dot(view_direction, reflect_direction), 0.0), 
                                 material.shininess);

                vec3 ambient = light.ambient * tex_diffuse;
                vec3 diffuse = light.diffuse * diff * tex_diffuse;
                vec3 specular = light.specular * spec * tex_specular; 

                float theta = dot(light_direction, normalize(-light.direction));
                float epsilon = light.cut_off - light.outer_cut_off;
                float intensity = clamp((theta - light.outer_cut_off) / epsilon, 0.0, 1.0);

                diffuse *= intensity;
                specular *= intensity;

                return (ambient + diffuse + specular);
            }


            vec3 calculate_point_light(PointLight light, vec3 normal, vec3 frag_pos, vec3 view_direction, vec3 tex_diffuse, vec3 tex_specular) {
                // diffuse
                vec3 light_direction = normalize(light.position - frag_pos);
                float diff = max(dot(normal, light_direction), 0.0);
    
                // specular
                vec3 reflect_direction = reflect(-light_direction, normal);  
                float spec = pow(max(dot(view_direction, reflect_direction), 0.0), 
                                 material.shininess);

                vec3 ambient = light.ambient * tex_diffuse;
                vec3 diffuse = light.diffuse * diff * tex_diffuse;
                vec3 specular = light.specular * spec * tex_specular; 

                float distance = length(light.position - frag_pos);
                float attenuation = 1.0 / (light.constant + light.linear * distance +
                                    light.quadratic * (distance * distance));
                   
                ambient *= attenuation;
                diffuse *= attenuation;
                specular *= attenuation;

                return (ambient + diffuse + specular);
            }


            void main() {

                // properties
                vec3 norm = normalize(v_normal);
                vec3 view_direction = normalize(view_pos - v_frag_pos);

                vec3 tex_diffuse = vec3(texture(material.diffuse, v_tex_coords));
                vec3 tex_specular = vec3(texture(material.specular, v_tex_coords));

                vec3 result = calculate_directional_light(directional_light, norm, view_direction, tex_diffuse, tex_specular);

                for(int i = 0; i < NR_POINT_LIGHTS; i++) {
                    result += calculate_point_light(point_lights[i], norm, v_frag_pos, view_direction, tex_diffuse, tex_specular);
                }

                result += calculate_spot_light(spot_light, norm, v_frag_pos, view_direction, tex_diffuse, tex_specular);
                
                out_color = vec4(result, 1.0);
            }
        "#;
        SimpleTexturedObjectProgram(
            Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap(),
        )
    }

    pub fn get_uniforms<'a, 'b, 'c, 'd, 'e, 'f, 'g>(
        matrix: &'a Matrix4<f32>,
        model: &'b Matrix4<f32>,
        material: &'c Material,
        directional_light: &'d DirectionalLight,
        spot_light: &'e SpotLight,
        point_lights: &'f [&'f PointLight; 4],
        view: &'g Point3<f32>,
    ) -> SimpleTexturedObjectUniforms<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
        SimpleTexturedObjectUniforms {
            matrix,
            model,
            material,
            directional_light,
            spot_light,
            point_lights,
            view,
        }
    }
}

pub struct SimpleLightObjectProgram(pub Program);

pub struct SimpleLightObjectUniforms<'a, 'b> {
    matrix: &'a Matrix4<f32>,
    color: &'b Vector3<f32>,
}

impl glium::uniforms::Uniforms for SimpleLightObjectUniforms<'_, '_> {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut f: F) {
        f("matrix", UniformValue::Mat4(self.matrix.to_array()));

        f(
            "color",
            UniformValue::Vec3([self.color.x, self.color.y, self.color.z]),
        );
    }
}

impl SimpleLightObjectProgram {
    pub fn new(display: &Display) -> SimpleLightObjectProgram {
        let vertex_shader_src = r#"
        #version 140

        in vec3 position;

        uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 1.0);
        }
    "#;

        let fragment_shader_src = r#"
        #version 140

        out vec4 out_color;

        uniform vec3 color;
        
        void main() {
            out_color = vec4(color, 1.0);
        }
    "#;

        SimpleLightObjectProgram(
            Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap(),
        )
    }

    pub fn get_uniforms<'a, 'b>(
        matrix: &'a Matrix4<f32>,
        color: &'b Vector3<f32>,
    ) -> SimpleLightObjectUniforms<'a, 'b> {
        SimpleLightObjectUniforms { matrix, color }
    }
}

pub struct MainFramebufferProgram(pub Program);

pub struct MainFramebufferProgramUniforms<'a> {
    screen_texture: &'a glium::texture::Texture2d,
}

impl glium::uniforms::Uniforms for MainFramebufferProgramUniforms<'_> {
    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut f: F) {
        f(
            "screen_texture",
            UniformValue::Texture2d(self.screen_texture, None),
        );
    }
}

pub enum PostProcessingEffects {
    NoPostProcessing,
    Inversed,
    GrayScale,
    DeepFried,
    Blur,
    Edged,
}

impl MainFramebufferProgram {
    pub fn new(
        display: &Display,
        post_processing_effect: &PostProcessingEffects,
    ) -> MainFramebufferProgram {
        let vertex_shader_src = r#"
            #version 330 core

            in vec2 tex_coords;
            in vec2 position;

            out vec2 v_tex_coords;

            void main() {
                gl_Position = vec4(position.x, position.y, 0.0, 1.0);
                v_tex_coords = tex_coords;
            }
        "#;

        // todo: fix weir border effects while using kernels

        let kernel_code;
        let main_code = match post_processing_effect {
            PostProcessingEffects::NoPostProcessing => {
                r#"
                void main() {
                    out_color = texture(screen_texture, v_tex_coords);
                }
            "#
            }
            PostProcessingEffects::Inversed => {
                r#"
                void main() {
                    out_color = vec4(vec3(1.0 - texture(screen_texture, v_tex_coords)), 1.0);
                }
            "#
            }
            PostProcessingEffects::GrayScale => {
                r#"
                void main() {
                    out_color = texture(screen_texture, v_tex_coords);
                    float average = 0.2126 * out_color.r + 0.7152 * out_color.g + 0.0722 * out_color.b;
                    out_color = vec4(average, average, average, 1.0);
                }
            "#
            }
            PostProcessingEffects::DeepFried => {
                kernel_code = from_kernel_template([
                    [-1.0, -1.0, -1.0],
                    [-1.0, 9.0, -1.0],
                    [-1.0, -1.0, -1.0],
                ]);
                &kernel_code
            }
            PostProcessingEffects::Blur => {
                kernel_code = from_kernel_template([
                    [1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0],
                    [2.0 / 16.0, 4.0 / 16.0, 2.0 / 16.0],
                    [1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0],
                ]);
                &kernel_code
            }
            PostProcessingEffects::Edged => {
                kernel_code =
                    from_kernel_template([[1.0, 1.0, 1.0], [1.0, -8.0, 1.0], [1.0, 1.0, 1.0]]);
                &kernel_code
            }
        };

        let fragment_shader_src = format!(
            r#"
            #version 330 core

            in vec2 v_tex_coords;

            out vec4 out_color;

            uniform sampler2D screen_texture;

            {}
        "#,
            main_code
        );

        MainFramebufferProgram(
            Program::from_source(display, vertex_shader_src, &fragment_shader_src, None).unwrap(),
        )
    }

    pub fn get_uniforms<'a, 'b>(
        screen_texture: &'a glium::texture::Texture2d,
    ) -> MainFramebufferProgramUniforms<'a> {
        MainFramebufferProgramUniforms { screen_texture }
    }
}

fn from_kernel_template(kernel: [[f32; 3]; 3]) -> String {
    let kernel_str = format!(
        r#"
        float kernel[9] = float[](
            {}, {}, {},
            {}, {}, {},
            {}, {}, {}
        );
    "#,
        kernel[0][0],
        kernel[0][1],
        kernel[0][2],
        kernel[1][0],
        kernel[1][1],
        kernel[1][2],
        kernel[2][0],
        kernel[2][1],
        kernel[2][2]
    );

    format!(
        r#"
    const float offset = 1.0 / 300.0;

    void main() {{
        vec2 offsets[9] = vec2[](
            vec2(-offset, offset), // top-left
            vec2( 0.0f, offset), // top-center
            vec2( offset, offset), // top-right
            vec2(-offset, 0.0f), // center-left
            vec2( 0.0f, 0.0f), // center-center
            vec2( offset, 0.0f), // center-right
            vec2(-offset, -offset), // bottom-left
            vec2( 0.0f, -offset), // bottom-center
            vec2( offset, -offset) // bottom-right
        );

        {}

        vec3 sampleTex[9];
        for(int i = 0; i < 9; i++) {{
            sampleTex[i] = vec3(texture(screen_texture, v_tex_coords.st +
            offsets[i]));
        }}
        vec3 col = vec3(0.0);
        for(int i = 0; i < 9; i++)
            col += sampleTex[i] * kernel[i];
        out_color = vec4(col, 1.0);
    }}
"#,
        kernel_str
    )
}
