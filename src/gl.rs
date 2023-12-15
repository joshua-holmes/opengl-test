use std::error::{Error, self};

use gl33::{
    self,
    GlFns,
};
use beryllium::{
    video::GlWindow,
};

/// The types of buffer object that you can have.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferType {
    /// Array Buffers holds arrays of vertex data for drawing.
    Array = gl33::GL_ARRAY_BUFFER.0 as isize,
    /// Element Array Buffers hold indexes of what vertexes to use for drawing.
    ElementArray = gl33::GL_ELEMENT_ARRAY_BUFFER.0 as isize,
}

pub struct Gl {
    gl_fns: GlFns,
}

impl Gl {
    pub fn new(gl_window_obj: GlWindow) -> Self {
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

    fn gen_vertex_arrays(&self, count: i32) -> Result<u32, &str> {
        let mut vao = 0;
        unsafe {
            self.gl_fns.GenVertexArrays(count, &mut vao);
        }
        if vao != 0 {
            Ok(vao)
        } else {
            Err("Could not create vertex array")
        }
    }

    fn bind_vertex_array(&self, vao_val: u32) {
        unsafe { self.gl_fns.BindVertexArray(vao_val) }
    }

    fn gen_buffers(&self, count: i32) -> Result<u32, &str> {
        let mut vbo = 0;
        unsafe {
            self.gl_fns.GenBuffers(count, &mut vbo);
        }
        if vbo != 0 {
            Ok(vbo)
        } else {
            Err("Could not create buffer object")
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
}

/// Basic wrapper for a [Vertex Array
/// Object](https://www.khronos.org/opengl/wiki/Vertex_Specification#Vertex_Array_Object).
pub struct VertexArray<'a> {
    val: u32,
    gl: &'a Gl,
}
impl<'a> VertexArray<'a> {
    /// Creates a new vertex array object
    pub fn new(gl: &Gl) -> Result<Self, &str> {
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
    pub fn new(gl: &Gl) -> Result<Self, &str> {
        Ok(Self { val: gl.gen_buffers(1)?, gl, buffer_type: None })
    }

    pub fn bind(&mut self, b_type: BufferType) {
        self.buffer_type = Some(b_type);
        self.gl.bind_buffer(b_type, self.val);
    }

    pub fn clear_binding(&self) -> Result<(), &str> {
        match self.buffer_type {
            Some(b_type) => {
                self.gl.bind_buffer(b_type, 0);
                Ok(())
            }
            None => Err("Could not clear buffer binding, buffer type not found")
        }
    }

    pub fn buffer_data(&self, data: &[u8], usage: gl33::GLenum) -> Result<(), &str> {
        match self.buffer_type {
            Some(b_type) => {
                self.gl.buffer_data(b_type, data, usage);
                Ok(())
            }
            None => Err("Could not set buffer data, buffer type not found")
        }
    }
}
