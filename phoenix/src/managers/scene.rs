use super::entity::{Entity, Manager};
use crate::components::color::RGBA;
use crate::renderer::Render;
use crate::window::{WinError, Window};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Window error: {0}")]
    WinError(#[from] WinError),
}

pub struct Scene {
    entity_manager: Manager,
    window: Window,
    renderer: Box<dyn Render>,
    background_color: RGBA,
}

impl Scene {
    #[must_use]
    pub fn new(window: Window, renderer: Box<dyn Render>) -> Self {
        Scene {
            entity_manager: Manager::default(),
            window,
            renderer,
            background_color: RGBA::default(),
        }
    }

    /// # Errors
    ///
    /// Returns Err when the window fails to set itself as the current window.
    pub fn start(&mut self) -> Result<()> {
        if !self.window.is_current() {
            self.window.set_current()?;
        }

        while self.window.is_running() {
            self.renderer.set_background_color(&self.background_color);

            let keys = self.entity_manager.get_keys();
            for key in keys {
                if let Ok(id) = self
                    .renderer
                    .init_entity(self.entity_manager.as_ref_entity(key))
                {
                    self.renderer.draw_entity(id);
                }
            }

            self.window.swap_buffers();
            Window::poll_events();
        }

        Ok(())
    }

    pub fn set_background_color(&mut self, color: RGBA) {
        self.background_color = color;
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entity_manager.add_entity(entity);
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod tests {
        use serial_test::serial;

        use crate::{
            components::{color::RGBA, shaders::shader_program::ShaderProgram},
            managers::{entity::View, scene::Scene},
            renderer::{Error, Render},
            window::{GlfwConfig, Resolution},
        };

        struct MockRenderer;

        impl MockRenderer {
            fn new() -> Self {
                MockRenderer
            }
        }

        impl Render for MockRenderer {
            fn set_background_color(&self, _color: &RGBA) {}

            fn init_entity(&mut self, _entity: View) -> Result<u32, Error> {
                todo!()
            }

            fn draw_entity(&self, _id: u32) {}

            fn compile_shader_program(
                &mut self,
                _shader_program: &ShaderProgram,
            ) -> crate::renderer::Result<crate::renderer::ID> {
                todo!()
            }
        }

        #[test]
        #[serial]
        fn test_scene_new() {
            let config = GlfwConfig::create().unwrap();
            let window = config.create_window("Test", Resolution::default()).unwrap();
            let renderer = Box::new(MockRenderer::new());
            let scene = Scene::new(window, renderer);

            assert_eq!(scene.background_color, RGBA::default());
        }
    }
}
