#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::mem;

use beryllium::{
    Sdl,
    init,
    video,
    events::Event,
};

use gl33::{
    GlFns,
};

type Vertex = [f32; 3];

const VERTICES: [Vertex; 3] = [
    [-0.5, -0.5, 0.0],
    [ 0.5, -0.5, 0.0],
    [ 0.0,  0.5, 0.0],
];
const WINDOW_TITLE: &str = "My triangle";
const VERT_SHADER: &str = r#"#version 330 core
layout (location = 0) in vec3 pos;
void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
}
"#;
const FRAG_SHADER: &str = r#"#version 330 core
out vec4 final_color;
void main() {
    final_color = vec4(1.0, 0.5, 0.2, 1.0);
}
"#;

fn main() {
    let sdl = Sdl::init(init::InitFlags::EVERYTHING);
    
    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();
    let mut flags = video::GlContextFlags::default();
    #[cfg(target_os = "macos")]
    {
        flags |= video::GlContextFlags::FORWARD_COMPATIBLE;
    }
    if cfg!(debug_asserts) {
        flags |= video::GlContextFlags::DEBUG;
    }
    sdl.set_gl_context_flags(flags).unwrap();
    let win_args = video::CreateWinArgs {
        title: WINDOW_TITLE,
        width: 800,
        height: 600,
        allow_high_dpi: true,
        borderless: false,
        resizable: false,
    };

    let win = sdl.create_gl_window(win_args).expect("big sad, no window");

    unsafe {
        let gl = GlFns::load_from(&|f_name| win.get_proc_address(f_name)).unwrap();
        gl.ClearColor(0.2, 0.3, 0.3, 1.0);

        let mut vao = 0;
        gl.GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);

        gl.BindBuffer(gl33::GL_ARRAY_BUFFER, vao);

        gl.BufferData(
            gl33::GL_ARRAY_BUFFER,
            mem::size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(),
            gl33::GL_STATIC_DRAW
        );

        gl.VertexAttribPointer(
            0,
            3,
            gl33::GL_FLOAT,
            gl33::GL_FALSE.0 as u8,
            mem::size_of::<Vertex>().try_into().unwrap(),
            0 as *const _
        );
        gl.EnableVertexAttribArray(0);

        let shader_opts = [
            ("Vertex", gl33::GL_VERTEX_SHADER, VERT_SHADER),
            ("Fragment", gl33::GL_FRAGMENT_SHADER, FRAG_SHADER)
        ];
        let mut shaders = Vec::with_capacity(shader_opts.len());

        let shader_program = gl.CreateProgram();
        for (shader_log_name, shader_type, source_code) in shader_opts {
            let shader = gl.CreateShader(shader_type);
            assert_ne!(shader, 0);

            gl.ShaderSource(
                shader,
                1,
                &(source_code.as_bytes().as_ptr().cast()),
                &(source_code.len().try_into().unwrap())
            );
            gl.CompileShader(shader);

            let mut success = 0;
            gl.GetShaderiv(shader, gl33::GL_COMPILE_STATUS, &mut success);

            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0;
                gl.GetShaderInfoLog(
                    shader,
                    1024,
                    &mut log_len,
                    v.as_mut_ptr().cast()
                );
                v.set_len(log_len.try_into().unwrap());
                panic!("{} Compile Error: {}", shader_log_name, String::from_utf8_lossy(&v));
            }

            gl.AttachShader(shader_program, shader);
            shaders.push(shader);
        } // shader for loop
        gl.LinkProgram(shader_program);

        let mut success = 0;
        gl.GetProgramiv(shader_program, gl33::GL_LINK_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl.GetProgramInfoLog(
                shader_program,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast()
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
        }

        for shader in shaders {
            gl.DeleteShader(shader);
        }
        gl.UseProgram(shader_program);

        win.set_swap_interval(video::GlSwapInterval::Vsync).unwrap();

        gl.Clear(gl33::GL_COLOR_BUFFER_BIT);

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
            gl.Clear(gl33::GL_COLOR_BUFFER_BIT);
            gl.DrawArrays(gl33::GL_TRIANGLES, 0, 3);

            win.swap_window();
        }
    } // unsafe
}
