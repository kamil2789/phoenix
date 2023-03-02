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

    pub fn remove_shader(&mut self, id: ShaderID) {
        if let Some(index) = self.shader_programs.iter().position(|&x| x == id) {
            let removed_id = self.shader_programs.swap_remove(index);
            self.graphic_api.delete_shader(removed_id);
        }
    }

    pub fn get_shaders_num(&self) -> usize {
        self.shader_programs.len()
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
    use crate::window::{create_window_lib_config, Library};
    use serial_test::serial;

    const VERTEX_SHADER_SRC: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
    "#;

    const FRAGMENT_SHADER_SRC: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
        FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
    "#;

    #[test]
    #[serial]
    fn test_shader_manager_compile_shader_error() {
        let config = create_window_lib_config(&Library::GLFW).unwrap();
        let window = config.create_default_window().unwrap();

        let graphic_api = create_graphic_api(&GraphicApiType::OpenGL, &window).unwrap();
        let mut manager = ShaderManager::new(graphic_api);
        let result = manager.compile_shader(" ", " ");
        assert!(result.is_err());
    }

    #[test]
    #[serial]
    fn test_shader_manager_compile_shader_ok() {
        let config = create_window_lib_config(&Library::GLFW).unwrap();
        let window = config.create_default_window().unwrap();

        let graphic_api = create_graphic_api(&GraphicApiType::OpenGL, &window).unwrap();
        let mut manager = ShaderManager::new(graphic_api);
        let result = manager.compile_shader(VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC);
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_shader_manager_delete_shader() {
        let config = create_window_lib_config(&Library::GLFW).unwrap();
        let window = config.create_default_window().unwrap();

        let graphic_api = create_graphic_api(&GraphicApiType::OpenGL, &window).unwrap();
        let mut manager = ShaderManager::new(graphic_api);
        let result = manager.compile_shader(VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC).unwrap();
        assert_eq!(1, manager.get_shaders_num());

        manager.remove_shader(result);
        assert_eq!(0, manager.get_shaders_num());
    }
}
