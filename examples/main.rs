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

        let vertex_shader = gl.CreateShader(gl33::GL_VERTEX_SHADER);
        assert_ne!(vertex_shader, 0);

        gl.ShaderSource(
            vertex_shader,
            1,
            &(VERT_SHADER.as_bytes().as_ptr().cast()),
            &(VERT_SHADER.len().try_into().unwrap())
        );
        gl.CompileShader(vertex_shader);

        let mut success = 0;
        gl.GetShaderiv(vertex_shader, gl33::GL_COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0;
            gl.GetShaderInfoLog(vertex_shader,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast()
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
        }
    }

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
    }
}
