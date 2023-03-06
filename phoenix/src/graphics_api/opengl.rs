mod shaders;

use crate::color::RGBA;
use crate::graphics_api::GraphicApi;
use crate::graphics_api::GraphicApiError;
use crate::graphics_api::{ShaderError, ShaderID};
use crate::graphics_api::PolygonError;
use crate::graphics_api::TriangleApi;
use crate::polygons::Triangle;
use crate::window::Window;

use gl;
use std::rc::Rc;

pub struct OpenGlApi {}

pub fn create_opengl_api(window: &Rc<dyn Window>) -> Result<OpenGlApi, GraphicApiError> {
    window.set_current();
    let result = OpenGlApi {};
    OpenGlApi::init()?;
    Ok(result)
}

struct OpenGlTriangle {
    vao: u32,
    vbo: u32,
    shader_program_id: u32
}

impl OpenGlApi {
    fn init() -> Result<(), GraphicApiError> {
        if gl_loader::init_gl() == 0 {
            return Err(GraphicApiError::InitApiError(String::from(
                "Cannot load openGL library",
            )));
        }

        gl::load_with(|symbol| gl_loader::get_proc_address(symbol).cast());

        if !gl::CreateProgram::is_loaded() || !gl::ClearColor::is_loaded() {
            Err(GraphicApiError::InitApiError(String::from(
                "Cannot load pointers to openGL functions.",
            )))
        } else {
            Ok(())
        }
    }
}

impl GraphicApi for OpenGlApi {
    fn draw_background(&self, color: &RGBA) {
        let normalized_color = color.get_as_normalized_f32();
        unsafe {
            gl::ClearColor(
                normalized_color[0],
                normalized_color[1],
                normalized_color[2],
                normalized_color[3],
            );
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    fn delete_shader(&self, id: u32) {
        shaders::delete(id);
    }

    fn compile_shader(
        &self,
        vertex_src: &str,
        fragment_src: &str,
    ) -> Result<ShaderID, ShaderError> {
        shaders::compile(vertex_src, fragment_src)
    }

    fn use_shader(&self, id: u32) {
        unsafe {
            gl::UseProgram(id);
        }
    }

    fn create_triangle(&self, triangle: &Triangle) -> Result<Rc<dyn TriangleApi>, PolygonError> {
        todo!()
    }
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
    fn init(&self) -> Result<(), PolygonError> {
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

impl Drop for OpenGlApi {
    fn drop(&mut self) {
        gl_loader::end_gl();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::window::{create_window_lib_config, Library};
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_create_opengl_api() {
        let config = create_window_lib_config(&Library::GLFW).unwrap();
        let window = config.create_default_window().unwrap();

        let result = create_opengl_api(&window);
        assert!(result.is_ok());
    }
}
