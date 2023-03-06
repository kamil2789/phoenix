use crate::graphics_api::{Result, GraphicApiError};
use crate::graphics_api::TriangleApi;

use gl;

struct OpenGlTriangle {
    vao: u32,
    vbo: u32,
    shader_program_id: u32
}

impl OpenGlTriangle {
    pub fn new() -> Self {
        OpenGlTriangle{vao: 0, vbo: 0, shader_program_id: 0}
    }

    fn generate_buffers(&mut self) {
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);
        }
    }

    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        }
    }
}

impl TriangleApi for OpenGlTriangle {
    fn init(&self) -> Result<()> {
        //self.generate_buffers();
        self.bind();
        //self.init_buffer(data)?;
        todo!()
    }

    fn draw(&self) {
        todo!()
    }
}

impl Drop for OpenGlTriangle {
    fn drop(&mut self) {
        
    }
}