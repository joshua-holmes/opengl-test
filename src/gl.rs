use std::fmt::Display;

use gl33::{
    self,
    GlFns,
};
use beryllium::video::GlWindow;

/// The types of buffer object that you can have.
#[derive(Clone, Copy)]
pub enum BufferType {
    /// Array Buffers holds arrays of vertex data for drawing.
    Array = gl33::GL_ARRAY_BUFFER.0 as isize,
    /// Element Array Buffers hold indexes of what vertexes to use for drawing.
    ElementArray = gl33::GL_ELEMENT_ARRAY_BUFFER.0 as isize,
}

#[derive(Clone, Copy)]
pub enum ShaderType {
    Vertex,
    Fragment,
}
impl Display for ShaderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            ShaderType::Vertex => "Vertex",
            ShaderType::Fragment => "Fragment",
        };

        write!(f, "{}", name)?;

        Ok(())
    }
}
impl ShaderType {
    pub fn to_glenum(&self) -> gl33::GLenum {
        match self {
            ShaderType::Vertex => gl33::GL_VERTEX_SHADER,
            ShaderType::Fragment => gl33::GL_FRAGMENT_SHADER,
        }
    }
}

#[derive(Clone, Copy)]
pub enum LogType {
    Shader,
    Program,
}

pub struct Gl {
    gl_fns: GlFns,
}

impl Gl {
    pub fn new(gl_window_obj: &GlWindow) -> Self {
        unsafe {
            let gl_fns = GlFns::load_from(&|f_name| gl_window_obj.get_proc_address(f_name)).unwrap();
            Self { gl_fns }
        }
    }

    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            self.gl_fns.ClearColor(r, g, b, a);
        }
    }

    fn gen_vertex_arrays(&self, count: i32) -> Result<u32, String> {
        let mut vao = 0;
        unsafe {
            self.gl_fns.GenVertexArrays(count, &mut vao);
        }
        if vao != 0 {
            Ok(vao)
        } else {
            Err("Could not create vertex array".to_string())
        }
    }

    fn bind_vertex_array(&self, vao_val: u32) {
        self.gl_fns.BindVertexArray(vao_val);
    }

    fn gen_buffers(&self, count: i32) -> Result<u32, String> {
        let mut vbo = 0;
        unsafe {
            self.gl_fns.GenBuffers(count, &mut vbo);
        }
        if vbo != 0 {
            Ok(vbo)
        } else {
            Err("Could not create buffer object".to_string())
        }
    }

    fn bind_buffer(&self, b_type: BufferType, vbo_val: u32) {
        unsafe { self.gl_fns.BindBuffer(gl33::GLenum(b_type as u32), vbo_val) }
    }

    fn buffer_data(&self, b_type: BufferType, data: &[u8], usage: gl33::GLenum) {
        unsafe {
            self.gl_fns.BufferData(
                gl33::GLenum(b_type as u32),
                data.len().try_into().unwrap(),
                data.as_ptr().cast(),
                usage,
            );
        }
    }

    pub fn vertex_attrib_pointer(&self, index: u32, size: i32, data_type: gl33::GLenum, normalized: bool, stride: usize) {
        unsafe {
            self.gl_fns.VertexAttribPointer(index, size, data_type, normalized as u8, stride.try_into().unwrap(), 0 as *const _);
        }
    }

    pub fn enable_vertex_attrib_array(&self, index: u32) {
        unsafe { self.gl_fns.EnableVertexAttribArray(index) }
    }

    pub fn create_program(&self) -> u32 {
        self.gl_fns.CreateProgram()
    }

    pub fn create_shader(&self, shader_type: ShaderType) -> Result<u32, String> {
        let shader = self.gl_fns.CreateShader(shader_type.to_glenum());
        if shader == 0 {
            Err(format!("Could not create shader of type: {}", shader_type))
        } else {
            Ok(shader)
        }
    }

    pub fn shader_source(&self, shader_data: u32, count: i32, source_code: String) {
        unsafe {
            self.gl_fns.ShaderSource(
                shader_data,
                count,
                &(source_code.as_bytes().as_ptr().cast()),
                &(source_code.len().try_into().unwrap())
            )
        }
    }

    fn compile_shader(&self, shader_data: u32) {
        self.gl_fns.CompileShader(shader_data);
    }

    pub fn get_program_iv(&self, shader_program: u32) -> Result<(), String> {
        let mut success = 0;
        unsafe { self.gl_fns.GetProgramiv(shader_program, gl33::GL_LINK_STATUS, &mut success) }
        if success == 0 {
            Err(self.get_info_log(LogType::Program, shader_program))
        } else {
            Ok(())
        }
    }

    fn get_shader_iv(&self, shader_data: u32) -> Result<(), String> {
        let mut success = 0;
        unsafe { self.gl_fns.GetShaderiv(shader_data, gl33::GL_COMPILE_STATUS, &mut success); }
        if success == 0 {
            Err(self.get_info_log(LogType::Shader, shader_data))
        } else {
            Ok(())
        }
    }

    fn get_info_log(&self, log_type: LogType, shader_data: u32) -> String {
        let mut v: Vec<u8> = Vec::with_capacity(1024);
        let mut log_len = 0;
        match log_type {
            LogType::Shader => unsafe {
                self.gl_fns.GetShaderInfoLog(
                    shader_data,
                    1024,
                    &mut log_len,
                    v.as_mut_ptr().cast()
                )
            }
            LogType::Program => unsafe {
                self.gl_fns.GetProgramInfoLog(
                    shader_data,
                    1024,
                    &mut log_len,
                    v.as_mut_ptr().cast()
                )
            }
        }
        unsafe {
            v.set_len(log_len.try_into().unwrap());
        }
        String::from_utf8_lossy(&v).to_string()
    }

    fn attach_shader(&self, shader_program: u32, shader_data: u32) {
        self.gl_fns.AttachShader(shader_program, shader_data);
    }

    pub fn link_program(&self, shader_program: u32) {
        self.gl_fns.LinkProgram(shader_program);
    }

    fn delete_shader(&self, shader_data: u32) {
        self.gl_fns.DeleteShader(shader_data);
    }

    pub fn use_program(&self, shader_program: u32) {
        self.gl_fns.UseProgram(shader_program);
    }

    pub fn clear(&self) {
        unsafe { self.gl_fns.Clear(gl33::GL_COLOR_BUFFER_BIT) }
    }

    pub fn draw_arrays(&self, index: usize, count: i32) {
        unsafe { self.gl_fns.DrawArrays(gl33::GL_TRIANGLES, index as i32, count) }
    }
}

/// Basic wrapper for a [Vertex Array
/// Object](https://www.khronos.org/opengl/wiki/Vertex_Specification#Vertex_Array_Object).
pub struct VertexArray<'a> {
    val: u32,
    gl: &'a Gl,
}
impl<'a> VertexArray<'a> {
    /// Creates a new vertex array object
    pub fn new(gl: &'a Gl) -> Result<Self, String> {
        Ok(Self { val: gl.gen_vertex_arrays(1)?, gl })
    }

    /// Bind this vertex array as the current vertex array object
    pub fn bind(&self) {
        self.gl.bind_vertex_array(self.val);
    }

    /// Clear the current vertex array object binding.
    pub fn clear_binding(&self) {
        self.gl.bind_vertex_array(0);
    }
}

pub struct Buffer<'a> {
    val: u32,
    gl: &'a Gl,
    buffer_type: Option<BufferType>,
}
impl<'a> Buffer<'a> {
    pub fn new(gl: &'a Gl) -> Result<Self, String> {
        Ok(Self { val: gl.gen_buffers(1)?, gl, buffer_type: None })
    }

    pub fn bind(&mut self, b_type: BufferType) {
        self.buffer_type = Some(b_type);
        self.gl.bind_buffer(b_type, self.val);
    }

    pub fn clear_binding(&self) -> Result<(), String> {
        match self.buffer_type {
            Some(b_type) => {
                self.gl.bind_buffer(b_type, 0);
                Ok(())
            }
            None => Err("Could not clear buffer binding, buffer type not found".to_string())
        }
    }

    pub fn buffer_data(&self, data: &[u8], usage: gl33::GLenum) -> Result<(), String> {
        match self.buffer_type {
            Some(b_type) => {
                self.gl.buffer_data(b_type, data, usage);
                Ok(())
            }
            None => Err("Could not set buffer data, buffer type not found".to_string())
        }
    }
}

pub struct Shader<'a> {
    shader_type: ShaderType,
    shader_data: u32,
    gl: &'a Gl,
}
impl<'a> Shader<'a> {
    pub fn new(shader_type: ShaderType, source_code: String, gl: &'a Gl) -> Result<Self, String> {
        let shader_data = gl.create_shader(shader_type)?;
        gl.shader_source(shader_data, 1, source_code);
        Ok(Self {
            shader_type,
            shader_data,
            gl,
        })
    }

    pub fn compile(&self) {
        self.gl.compile_shader(self.shader_data);
    }

    pub fn get_shader_iv(&self) -> Result<(), String> {
        self.gl.get_shader_iv(self.shader_data)
    }

    pub fn attach(&self, shader_program: u32) {
        self.gl.attach_shader(shader_program, self.shader_data);
    }

    pub fn delete(&self) {
        self.gl.delete_shader(self.shader_data);
    }
}
