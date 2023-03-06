mod opengl;

use crate::color::RGBA;
use crate::graphics_api::opengl::create_opengl_api;
use crate::window::Window;
use crate::polygons::Triangle;
use thiserror::Error;
use std::rc::Rc;

pub type ShaderID = u32;
type Result<T> = std::result::Result<T, GraphicApiError>;

pub enum GraphicApiType {
    OpenGL,
    Vulkan,
}

#[derive(Error, Debug)]
pub enum GraphicApiError {
    #[error("Init API error: {0}")]
    InitApiError(String),
    #[error("Shader compile error: {0}")]
    ShaderCompileError(String),
    #[error("Shader linker error: {0}")]
    ShaderLinkError(String),
    #[error("Polygon error: {0}")]
    PolygonError(String)
}

pub trait GraphicApi {
    fn draw_background(&self, color: &RGBA);
    fn delete_shader(&self, id: ShaderID);
    /// # Errors
    fn compile_shader(&self, vertex_src: &str, fragment_src: &str)
        -> Result<ShaderID>;
    fn use_shader(&self, id: ShaderID);
    fn create_triangle(&self, triangle: &Triangle) -> Result<Rc<dyn TriangleApi>>;
}

pub trait TriangleApi: Drop {
    fn init(&self) -> Result<()>;
    fn draw(&self);
}

/// # Errors
pub fn create_graphic_api(
    graphic_api_type: &GraphicApiType,
    window: &Rc<dyn Window>,
) -> Result<Rc<dyn GraphicApi>> {
    match graphic_api_type {
        GraphicApiType::OpenGL => {
            let result = create_opengl_api(window)?;
            Ok(Rc::new(result))
        }
        GraphicApiType::Vulkan => unimplemented!(),
    }
}
