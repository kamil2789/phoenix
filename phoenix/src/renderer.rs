pub mod opengl;
pub mod vulkan;

use crate::{
    components::{color::RGBA, shaders::shader_program::ShaderProgram},
    managers::entity::ComponentRefs,
};
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
    /// # Errors
    ///
    /// Will return `Err` when the shader fails in the compilation or linking phase.
    /// The correct vertex and fragment shader should be given to this func.
    fn compile_shader_program(&mut self, shader_program: &ShaderProgram) -> Result<ID>;
    fn set_background_color(&self, color: &RGBA);
    /// # Errors
    ///
    /// Will return `Err` when shader compilation failed.
    fn init_entity(&mut self, entity: ComponentRefs) -> Result<ID>;
    fn draw_entity(&self, entity_id: ID);
}
