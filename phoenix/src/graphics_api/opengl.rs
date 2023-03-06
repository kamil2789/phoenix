mod shaders;
mod triangle;
mod rendering;

use crate::color::RGBA;
use crate::graphics_api::GraphicApi;
use crate::graphics_api::{Result, GraphicApiError};
use crate::graphics_api::{ShaderID, TriangleApi};
use crate::polygons::Triangle;
use crate::window::Window;

use gl;
use std::rc::Rc;

pub struct OpenGlApi {}

pub fn create_opengl_api(window: &Rc<dyn Window>) -> Result<OpenGlApi> {
    window.set_current();
    let result = OpenGlApi {};
    OpenGlApi::init()?;
    Ok(result)
}

impl OpenGlApi {
    fn init() -> Result<()> {
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
        rendering::draw_background(color);
    }

    fn delete_shader(&self, id: u32) {
        shaders::delete(id);
    }

    fn compile_shader(
        &self,
        vertex_src: &str,
        fragment_src: &str,
    ) -> Result<ShaderID> {
        shaders::compile(vertex_src, fragment_src)
    }

    fn use_shader(&self, id: u32) {
        shaders::use_shader(id);
    }

    fn create_triangle(&self, triangle: &Triangle) -> Result<Rc<dyn TriangleApi>> {
        todo!()
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
