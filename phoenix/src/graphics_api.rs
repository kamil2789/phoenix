mod opengl;

use crate::color::RGBA;
use crate::graphics_api::opengl::create_opengl_api;
use crate::window::Window;
use crate::polygons::Triangle;

use std::rc::Rc;

pub type ShaderID = u32;

#[derive(Debug)]
pub enum ShaderError {
    CompileError(String),
    LinkError(String),
}

#[derive(Debug)]
pub enum PolygonError {

}

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
    fn delete_shader(&self, id: ShaderID);
    /// # Errors
    fn compile_shader(&self, vertex_src: &str, fragment_src: &str)
        -> Result<ShaderID, ShaderError>;
    fn use_shader(&self, id: ShaderID);
    fn create_triangle(&self, triangle: &Triangle) -> Result<Rc<dyn TriangleApi>, PolygonError>;
}

pub trait TriangleApi: Drop {
    fn init(&self) -> Result<(), PolygonError>;
    fn draw(&self);
}

/// # Errors
pub fn create_graphic_api(
    graphic_api_type: &GraphicApiType,
    window: &Rc<dyn Window>,
) -> Result<Rc<dyn GraphicApi>, GraphicApiError> {
    match graphic_api_type {
        GraphicApiType::OpenGL => {
            let result = create_opengl_api(window)?;
            Ok(Rc::new(result))
        }
        GraphicApiType::Vulkan => unimplemented!(),
    }
}
