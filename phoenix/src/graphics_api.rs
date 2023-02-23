mod opengl;

use crate::color::RGBA;
use crate::graphics_api::opengl::create_opengl_api;

use std::rc::Rc;

pub enum GraphicApiType {
    OpenGL,
    Vulkan,
}

#[derive(Debug)]
pub enum GraphicApiError {
    InitApiError(String),
}

pub trait GraphicApi {
    fn draw_background(&self, color: &RGBA);
}

/// # Errors
pub fn create_graphic_api(
    graphic_api_type: &GraphicApiType,
) -> Result<Rc<dyn GraphicApi>, GraphicApiError> {
    match graphic_api_type {
        GraphicApiType::OpenGL => {
            let result = create_opengl_api()?;
            Ok(Rc::new(result))
        }
        GraphicApiType::Vulkan => unimplemented!(),
    }
}
