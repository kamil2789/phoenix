mod event_interpreter;
use std::rc::Rc;

use cgmath::{Vector3, Vector4};

use super::camera::{Camera, Config};
use super::performance::{FpsCounter, GlfwTimer};
use crate::components::color::{Color, RGBA};
use crate::components::transformer::Transformer;
use crate::components::Shape;
use crate::entities::entity::{Entity, Manager, View};
use crate::renderer::{self, Render};
use crate::window::{WinError, Window};
use crate::{entities, events};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Window error: {0}")]
    WinError(#[from] WinError),
    #[error("Renderer error: {0}")]
    RendererError(#[from] renderer::Error),
    #[error("Event error: {0}")]
    EventError(#[from] events::Error),
}

pub struct Scene {
    entity_manager: Manager,
    window: Rc<Window>,
    renderer: Box<dyn Render>,
    background_color: RGBA,
    camera: Option<Camera>,
    pub event_manager: events::Manager,
    fps_counter: FpsCounter,
}

impl Scene {
    #[must_use]
    pub fn new(window: Rc<Window>, renderer: Box<dyn Render>) -> Self {
        Scene {
            entity_manager: Manager::default(),
            window: window.clone(),
            renderer,
            background_color: RGBA::default(),
            camera: None,
            event_manager: events::Manager::new(window),
            fps_counter: FpsCounter::new(Box::new(GlfwTimer::default())),
        }
    }

    /// # Errors
    ///
    /// Returns Err when the window fails to set itself as the current window.
    /// Returns Err when rendering in particular frame fails.
    pub fn start(&mut self) -> Result<()> {
        if !self.window.is_current() {
            self.window.set_current();
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
            self.window.set_current();
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

    pub fn enable_3d(&mut self) {
        self.renderer.enable_3d();
    }

    /// # Errors
    ///
    /// Returns Err when the window fails to set itself as the current window.
    pub fn set_current_window(&self) -> Result<()> {
        self.window.set_current();
        Ok(())
    }

    #[must_use]
    pub fn get_delta_time(&self) -> f32 {
        self.fps_counter.get_delta_time()
    }

    #[must_use]
    pub fn get_last_error_code(&self) -> Option<u32> {
        self.renderer.get_last_error_code()
    }

    fn frame(&mut self) -> Result<()> {
        self.fps_counter.update();
        self.renderer.set_background_color(&self.background_color);

        self.handle_user_input_callbacks();

        //light variable should be updated before handling shapes
        if let Some(light_entity) = self.entity_manager.get_light_entity() {
            light_entity.shape.unwrap();
            let camera_pos = self.camera.as_ref().map_or_else(
                || Vector3::new(0.0, 0.0, 0.0),
                |cam| cam.get_camera_vec_pos(),
            );
            self.renderer.update_light_uniform_variables(
                &camera_pos,
                &self.get_light_pos(light_entity.shape.unwrap()),
                &self.get_light_color(light_entity.color.unwrap()),
            );
        }

        let keys = self.entity_manager.get_keys();
        for key in keys {
            let entity_view = self.entity_manager.as_ref_entity(key);
            let id = self.renderer.init_entity(entity_view.clone())?; //TODO make it a ref

            self.handle_entity_transformation(entity_view.clone())?;
            self.handle_camera(key)?; //out of loop?
            self.renderer
                .update_default_shader_uniform_variables(&self.entity_manager.as_ref_entity(key))?;

            //final step to draw the entity
            self.renderer.draw_entity(id);
        }

        self.window.swap_buffers();
        Window::poll_events();
        Ok(())
    }

    fn handle_user_input_callbacks(&mut self) {
        event_interpreter::process_actions(self.event_manager.process_events(), self);
    }

    fn get_light_pos(&self, shape: &dyn Shape) -> Vector3<f32> {
        let data = shape.get_vertices();
        Vector3::new(data[0], data[1], data[2])
    }

    fn get_light_color(&self, color: &Color) -> Vector4<f32> {
        let rgba = color.as_ref_uniform().unwrap().get_rgba();
        Vector4::new(rgba.0.into(), rgba.1.into(), rgba.2.into(), rgba.3)
    }

    fn handle_entity_transformation(&self, entity: View) -> Result<()> {
        if let Some(transformer) = entity.transformer {
            self.renderer
                .perform_transformations(entity.entity_id, transformer)?;
        } else {
            self.renderer
                .perform_transformations(entity.entity_id, &Transformer::new_identity())?;
        }

        Ok(())
    }

    fn handle_camera(&self, entity_id: u32) -> Result<()> {
        if let Some(cam) = &self.camera {
            self.renderer
                .perform_camera_position_transformation(entity_id, &cam.get_camera_position())?;
            self.renderer
                .perform_camera_projection_transformation(entity_id, &cam.get_projection())?;
        }

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
                geometry::plane::Triangle,
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
                &self,
                _entity_id: ID,
                _transformation: &crate::components::transformer::Transformer,
            ) -> crate::renderer::Result<()> {
                Ok(())
            }

            fn perform_camera_projection_transformation(
                &self,
                _entity_id: ID,
                _camera_matrix: &Matrix4<f32>,
            ) -> crate::renderer::Result<()> {
                todo!()
            }

            fn perform_camera_position_transformation(
                &self,
                _entity_id: ID,
                _camera_matrix: &Matrix4<f32>,
            ) -> crate::renderer::Result<()> {
                todo!()
            }

            fn update_default_shader_uniform_variables(
                &self,
                _entity: &View,
            ) -> crate::renderer::Result<()> {
                Ok(())
            }

            fn enable_3d(&self) {
                todo!()
            }

            fn get_last_error_code(&self) -> Option<u32> {
                todo!()
            }

            fn update_light_uniform_variables(
                &self,
                camera_pos: &cgmath::Vector3<f32>,
                light_pos: &cgmath::Vector3<f32>,
                light_color: &cgmath::Vector4<f32>,
            ) {
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
