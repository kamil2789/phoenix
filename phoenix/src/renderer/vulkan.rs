use crate::components::material::Material;

use super::{Api, Render};

#[derive(Clone)]
pub struct Vulkan {}

impl Vulkan {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Vulkan {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for Vulkan {
    fn compile_shader_program(
        &mut self,
        _shader_program: std::rc::Rc<crate::components::shaders::ShaderSource>,
    ) -> super::Result<super::ID> {
        todo!()
    }

    fn set_background_color(&self, _color: &crate::components::color::RGBA) {
        todo!()
    }

    fn get_api(&self) -> super::Api {
        Api::Vulkan
    }

    fn init_entity(&mut self, _entity: &crate::entities::entity::View) -> super::Result<super::ID> {
        todo!()
    }

    fn init_texture(
        &mut self,
        _texture: &crate::components::texture::Texture,
    ) -> super::Result<super::ID> {
        todo!()
    }

    fn perform_transformations(
        &self,
        _entity_id: super::ID,
        _transformation: &crate::components::transformer::Transformer,
    ) -> super::Result<()> {
        todo!()
    }

    fn perform_camera_projection_transformation(
        &self,
        _entity_id: super::ID,
        _camera_matrix: &cgmath::Matrix4<f32>,
    ) -> super::Result<()> {
        todo!()
    }

    fn perform_camera_position_transformation(
        &self,
        _entity_id: super::ID,
        _camera_matrix: &cgmath::Matrix4<f32>,
    ) -> super::Result<()> {
        todo!()
    }

    fn update_default_shader_uniform_variables(
        &self,
        _entity: &crate::entities::entity::View,
    ) -> super::Result<()> {
        todo!()
    }

    fn update_light_uniform_struct(
        &self,
        _entity_id: u32,
        _light: &crate::components::light::Light,
        _light_position: &cgmath::Vector3<f32>,
    ) -> super::Result<()> {
        todo!()
    }

    fn update_camera_position_vec(
        &self,
        _entity_id: super::ID,
        _camera_position: &cgmath::Vector3<f32>,
    ) -> super::Result<()> {
        todo!()
    }

    fn update_material_uniform_struct(
        &self,
        _entity_id: super::ID,
        _material: &Material,
    ) -> super::Result<()> {
        todo!()
    }

    fn draw_entity(&self, _entity_id: super::ID) {
        todo!()
    }

    fn enable_3d(&self) {
        todo!()
    }

    fn get_last_error_code(&self) -> Option<u32> {
        todo!()
    }
}
