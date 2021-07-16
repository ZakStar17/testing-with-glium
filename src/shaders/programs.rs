use glium::{Display, Program};

pub fn simple_textured_object(display: &Display) -> Program {
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
        
        uniform sampler2D tex;

        // I can't find a way to pass uniforms as structs
        uniform vec3 material_diffuse;
        uniform vec3 material_specular;
        uniform float material_shininess;

        uniform vec3 light_pos;
        uniform vec3 light_diffuse;
        uniform vec3 light_specular;

        uniform vec3 ambient_color;
        uniform vec3 view_pos;
        
        void main() {
            // diffuse
            vec3 norm = normalize(v_normal);
            vec3 light_direction = normalize(light_pos - v_frag_pos);
            float diff = max(dot(norm, light_direction), 0.0);

            // specular
            vec3 view_direction = normalize(view_pos - v_frag_pos);
            vec3 reflect_direction = reflect(-light_direction, norm);  
            float spec = pow(max(dot(view_direction, reflect_direction), 0.0), 
                             material_shininess);

            vec3 ambient = ambient_color;
            vec3 diffuse = light_diffuse * (diff * material_diffuse);
            vec3 specular = light_specular * (spec * material_specular); 

            out_color = texture(tex, v_tex_coords) * vec4((ambient + diffuse + specular), 1.0);
        }
    "#;
    //((texture(tex, v_tex_coords) * vec4(8.0, 8.0, 8.0, 1.0)) - vec4(1.9, 1.9, 1.9, 0.0)) * vec4(0.125, 0.125, 0.125, 1.0)

    Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap()
}

pub fn simple_light_object(display: &Display) -> Program {
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

    Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap()
}