pub mod opengl;
pub(crate) mod shaders;
pub mod vulkan;

use std::rc::Rc;

use crate::{
    components::{color::RGBA, shaders::ShaderSource, texture::Texture, transformer::Transformer},
    entities::entity::View,
};
use cgmath::{Matrix4, Vector3, Vector4};
use thiserror::Error;

pub type ID = u32;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Shader compilation error {0}")]
    CompilationError(String),
    #[error("Shader linking program error {0}")]
    LinkError(String),
    #[error("Rendering error {0}")]
    RenderingError(String),
    #[error("Transformation error {0}")]
    TransformationError(String),
}

pub trait Render {
    /// # Errors
    ///
    /// Will return `Err` when the shader fails in the compilation or linking phase.
    /// The correct vertex and fragment shader should be given to this func.
    fn compile_shader_program(&mut self, shader_program: Rc<ShaderSource>) -> Result<ID>;
    fn set_background_color(&self, color: &RGBA);
    /// # Errors
    ///
    /// Will return `Err` when shader compilation failed.
    fn init_entity(&mut self, entity: View) -> Result<ID>;
    /// # Errors
    ///
    /// Will return `Err` when texture initialization failed.
    fn init_texture(&mut self, texture: &Texture) -> Result<ID>;
    /// # Errors
    ///
    /// Will return `Err` when transformation failed or cannot be applied.
    fn perform_transformations(&self, entity_id: ID, transformation: &Transformer) -> Result<()>;
    /// # Errors
    ///
    /// Will return `Err` when transformation failed or cannot be applied.
    fn perform_camera_projection_transformation(
        &self,
        entity_id: ID,
        camera_matrix: &Matrix4<f32>,
    ) -> Result<()>;
    /// # Errors
    ///
    /// Will return `Err` when transformation failed or cannot be applied.
    fn perform_camera_position_transformation(
        &self,
        entity_id: ID,
        camera_matrix: &Matrix4<f32>,
    ) -> Result<()>;
    /// # Errors
    ///
    /// Will return `Err` when uniform variables cannot be set.
    fn update_default_shader_uniform_variables(&self, entity: &View) -> Result<()>;
    fn update_light_uniform_variables(
        &self,
        camera_pos: &Vector3<f32>,
        light_pos: &Vector3<f32>,
        light_color: &Vector4<f32>,
    );
    fn draw_entity(&self, entity_id: ID);
    fn enable_3d(&self);
    fn get_last_error_code(&self) -> Option<u32>;
}
