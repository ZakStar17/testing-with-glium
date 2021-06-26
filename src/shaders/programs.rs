
use glium::{Display, Program};

pub fn simple_textured_object(display: &Display) -> Program {
    let vertex_shader_src = r#"
        #version 140

        in vec3 position;
        in vec2 tex_coords;

        out vec2 v_tex_coords;

        uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 1.0);
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
        }
    "#;

    Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap()
}