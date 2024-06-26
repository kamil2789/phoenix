use std::rc::Rc;

use super::camera::{Camera, Config};
use crate::components::color::RGBA;
use crate::entities;
use crate::entities::entity::{Entity, Manager};
use crate::renderer::{self, Render};
use crate::window::{WinError, Window};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Window error: {0}")]
    WinError(#[from] WinError),
    #[error("Renderer error: {0}")]
    RendererError(#[from] renderer::Error),
}

pub struct Scene {
    entity_manager: Manager,
    window: Rc<Window>,
    renderer: Box<dyn Render>,
    background_color: RGBA,
    camera: Option<Camera>,
}

impl Scene {
    #[must_use]
    pub fn new(window: Rc<Window>, renderer: Box<dyn Render>) -> Self {
        Scene {
            entity_manager: Manager::default(),
            window,
            renderer,
            background_color: RGBA::default(),
            camera: None,
        }
    }

    /// # Errors
    ///
    /// Returns Err when the window fails to set itself as the current window.
    /// Returns Err when rendering in particular frame fails.
    pub fn start(&mut self) -> Result<()> {
        if !self.window.is_current() {
            self.window.set_current()?;
        }

        while self.window.is_running() {
            self.frame()?;
        }

        Ok(())
    }

    pub fn set_background_color(&mut self, color: RGBA) {
        self.background_color = color;
    }

    pub fn add_entity(&mut self, entity: Entity) {
        let result = entities::preprocessing::preprocessing(entity);
        self.entity_manager.add_entity(result);
    }

    /// # Errors
    ///
    /// Returns Err when the window fails to set itself as the current window.
    pub fn start_one_frame(&mut self) -> Result<()> {
        if !self.window.is_current() {
            self.window.set_current()?;
        }

        self.frame()
    }

    #[must_use]
    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn register_camera(&mut self, camera_config: &Config) {
        self.camera = Some(Camera::new(&self.window.get_resolution(), camera_config));
    }

    fn frame(&mut self) -> Result<()> {
        self.renderer.set_background_color(&self.background_color);

        let keys = self.entity_manager.get_keys();
        for key in keys {
            let id = self
                .renderer
                .init_entity(self.entity_manager.as_ref_entity(key))?;
            if let Some(transformer) = self.entity_manager.as_ref_transformers(key) {
                self.renderer.perform_transformations(id, transformer)?;
            }

            if let Some(cam) = &self.camera {
                self.renderer
                    .perform_camera_transformation(id, cam.get_projection())?;
            }
            self.renderer.draw_entity(id);
        }

        self.window.swap_buffers();
        Window::poll_events();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod tests {
        use std::rc::Rc;

        use cgmath::Matrix4;
        use serial_test::serial;

        use crate::{
            components::{
                color::{Color, RGBA},
                geometry::Triangle,
                shaders::ShaderSource,
                texture::Texture,
                Component,
            },
            entities::entity::{Entity, View},
            renderer::{Error, Render, ID},
            systems::scene::Scene,
            window::{GlfwConfig, Resolution},
        };

        struct MockRenderer {
            entities: u32,
        }

        impl MockRenderer {
            fn new() -> Self {
                MockRenderer { entities: 0 }
            }
        }

        impl Render for MockRenderer {
            fn set_background_color(&self, _color: &RGBA) {}

            fn init_entity(&mut self, _entity: View) -> Result<u32, Error> {
                self.entities += 1;
                Ok(self.entities)
            }

            fn draw_entity(&self, _id: u32) {}

            fn compile_shader_program(
                &mut self,
                _shader_program: Rc<ShaderSource>,
            ) -> crate::renderer::Result<crate::renderer::ID> {
                todo!()
            }

            fn init_texture(&mut self, _texture: &Texture) -> crate::renderer::Result<ID> {
                todo!()
            }

            fn perform_transformations(
                &mut self,
                _entity_id: ID,
                _transformation: &crate::components::transformer::Transformer,
            ) -> crate::renderer::Result<()> {
                Ok(())
            }

            fn perform_camera_transformation(
                &mut self,
                _entity_id: ID,
                _camera_matrix: &Matrix4<f32>,
            ) -> crate::renderer::Result<()> {
                todo!()
            }
        }

        #[test]
        #[serial]
        fn test_scene_new() {
            let config = GlfwConfig::create().unwrap();
            let window = Rc::new(config.create_window("Test", Resolution::default()).unwrap());
            let renderer = Box::new(MockRenderer::new());
            let scene = Scene::new(window, renderer);

            assert_eq!(scene.background_color, RGBA::default());
        }

        #[test]
        #[serial]
        fn test_scene_start_one_frame_no_entities() {
            let config = GlfwConfig::create().unwrap();
            let window = Rc::new(config.create_window("Test", Resolution::default()).unwrap());
            let renderer = Box::new(MockRenderer::new());

            let mut scene = Scene::new(window, renderer);
            assert!(scene.start_one_frame().is_ok());
        }

        #[test]
        #[serial]
        fn test_scene_start_one_frame() {
            let config = GlfwConfig::create().unwrap();
            let window = Rc::new(config.create_window("Test", Resolution::default()).unwrap());
            let renderer = Box::new(MockRenderer::new());

            let mut scene = Scene::new(window, renderer);

            let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

            let triangle = Triangle::new(vertices);
            let mut entity = Entity::default();
            entity.add_component(Component::Geometry(Box::new(triangle)));
            entity.add_component(Component::Color(Color::from_hex(0xFF_00_00_FF)));

            scene.add_entity(entity);
            assert!(scene.start_one_frame().is_ok());
        }
    }
}
