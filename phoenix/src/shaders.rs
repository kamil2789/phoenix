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
    use crate::window::{create_window_lib_config, Library};
    use serial_test::serial;

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
        let vertex_shader_src: &str = r#"
            #version 330 core
            layout (location = 0) in vec3 aPos;
            void main() {
                gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
            }
            "#;

        let fragment_shader_src: &str = r#"
            #version 330 core
            out vec4 FragColor;
            void main() {
                FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
            }
            "#;

        let config = create_window_lib_config(&Library::GLFW).unwrap();
        let window = config.create_default_window().unwrap();

        let graphic_api = create_graphic_api(&GraphicApiType::OpenGL, &window).unwrap();
        let mut manager = ShaderManager::new(graphic_api);
        let result = manager.compile_shader(vertex_shader_src, fragment_shader_src);
        assert!(result.is_ok());
    }
}
