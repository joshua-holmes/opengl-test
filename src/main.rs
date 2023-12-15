#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::mem;

use beryllium::{
    video,
    events::Event,
};

mod gl;
mod window;

use gl::{Gl, Buffer, BufferType, ShaderType, Shader};

use crate::gl::VertexArray;

type Vertex = [f32; 3];

const VERTICES: [Vertex; 3] = [
    [-0.5, -0.5, 0.0],
    [ 0.5, -0.5, 0.0],
    [ 0.0,  0.5, 0.0],
];
const WINDOW_TITLE: &str = "My triangle";
const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
const VERT_SHADER: String = String::from(r#"#version 330 core
layout (location = 0) in vec3 pos;
void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
}
"#);
const FRAG_SHADER: String = String::from(r#"#version 330 core
out vec4 final_color;
void main() {
    final_color = vec4(1.0, 0.5, 0.2, 1.0);
}
"#);

fn main() -> Result<(), &'static str> {
    let win = window::create_gl_window_obj(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT);

    unsafe {
        let gl = Gl::new(win);
        gl.clear_color(0.2, 0.3, 0.3, 1.0);

        let vao = VertexArray::new(&gl).unwrap();
        vao.bind();

        let vbo = Buffer::new(&gl).unwrap();
        vbo.bind(BufferType::Array);
        vbo.buffer_data(
            bytemuck::cast_slice(&VERTICES),
            gl33::GL_STATIC_DRAW,
        )?;

        gl.vertex_attrib_pointer(
            0,
            3,
            gl33::GL_FLOAT,
            false,
            mem::size_of::<Vertex>(),
        );
        gl.enable_vertex_attrib_array(0);

        let shader_program = gl.create_program();
        let shaders = [
            Shader::new(ShaderType::Vertex, VERT_SHADER, &gl)?,
            Shader::new(ShaderType::Fragment, FRAG_SHADER, &gl)?,
        ];

        for shader in shaders {
            shader.compile();
            shader.get_shader_iv()?;
            shader.attach(shader_program);
        }

        gl.link_program(shader_program);
        gl.get_program_iv(shader_program);

        for shader in shaders {
            shader.delete();
        }

        gl.use_program(shader_program);

        win.set_swap_interval(video::GlSwapInterval::Vsync).unwrap();

        gl.clear();

        'main_loop: loop {
            // handle events this frame
            while let Some((event, _timestamp)) = sdl.poll_events() {
                match event {
                    Event::Quit => break 'main_loop,
                    _ => (),
                }
            }
            // now the events are clear

            // here's where we could change the world state and draw
            gl.clear();
            gl.draw_arrays(0, 3);

            win.swap_window();
        }
    } // unsafe

    Ok(())
}
