mod shaders;

use crate::color::RGBA;
use crate::graphics_api::GraphicApi;
use crate::graphics_api::GraphicApiError;
use crate::graphics_api::{ShaderError, ShaderID};

use gl;

pub struct OpenGlApi {}

pub fn create_opengl_api() -> Result<OpenGlApi, GraphicApiError> {
    let result = OpenGlApi {};
    OpenGlApi::init()?;
    Ok(result)
}

impl OpenGlApi {
    fn init() -> Result<(), GraphicApiError> {
        if gl_loader::init_gl() == 0 {
            return Err(GraphicApiError::InitApiError(String::from(
                "Cannot load openGL library",
            )));
        }
        gl::load_with(|symbol| gl_loader::get_proc_address(symbol).cast());
        Ok(())
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;


    #[test]
    #[serial]
    fn test_create_opengl_api() {
        let result = create_opengl_api();
        assert!(result.is_ok());
    }

}
