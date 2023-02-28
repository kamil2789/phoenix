use crate::graphics_api::GraphicApi;
use crate::graphics_api::ShaderError;
use crate::graphics_api::ShaderID;
use std::rc::Rc;

pub struct ShaderManager {
    shader_programs: Vec<ShaderID>,
    graphic_api: Rc<dyn GraphicApi>,
}

impl ShaderManager {
    pub fn new(graphic_api: Rc<dyn GraphicApi>) -> Self {
        ShaderManager {
            shader_programs: vec![],
            graphic_api,
        }
    }

    /// # Errors
    pub fn compile_shader(
        &mut self,
        vertex_src: &str,
        fragment_src: &str,
    ) -> Result<ShaderID, ShaderError> {
        let id = self.graphic_api.compile_shader(vertex_src, fragment_src)?;
        self.shader_programs.push(id);
        Ok(id)
    }

    pub fn remove_shader(&self, id: ShaderID) {
        self.graphic_api.delete_shader(id);
    }
}

impl Drop for ShaderManager {
    fn drop(&mut self) {
        self.shader_programs
            .iter()
            .for_each(|id| self.graphic_api.delete_shader(*id));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics_api::{create_graphic_api, GraphicApiType};

    #[test]
    #[ignore]
    fn test_shader_manager_compile_shader() {
        let graphic_api = create_graphic_api(&GraphicApiType::OpenGL).unwrap();
        let mut manager = ShaderManager::new(graphic_api);
        let result = manager.compile_shader(" ", " ");
        assert!(result.is_err());
    }
}
