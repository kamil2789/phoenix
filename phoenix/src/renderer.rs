pub mod opengl;
pub mod vulkan;

use crate::components::{geometry::Shape, shaders::shader_program::ShaderProgram};
use thiserror::Error;

pub type ID = u32;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Shader compilation error {0}")]
    CompilationError(String),
    #[error("Shader linking program error {0}")]
    LinkError(String),
}

pub trait Render {
    fn compile_shader_program(&mut self, shader_program: &ShaderProgram) -> Result<ID>;
    //fn init_shape(&mut self, shape: impl Shape) -> Result<ID>;
    //fn init_triangle(triangle: &mut Triangle) -> Result<Box<dyn Shape>>;
    //fn draw_triangle(triangle: &Triangle) -> Result<>;
    //fn remove_triangle(triangle: &mut Triangle);
}
