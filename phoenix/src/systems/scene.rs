mod event_interpreter;
use std::rc::Rc;

use super::camera::{Camera, Config};
use super::lighting::{calculate_light, LightConfig};
use super::performance::{FpsCounter, GlfwTimer};
use crate::components::color::RGBA;
use crate::components::transformer::Transformer;
use crate::entities::entity::{Entity, Manager, View};
use crate::renderer::{self, Render};
use crate::window::{WinError, Window};
use crate::{entities, events};

pub type Result<T> = std::result::Result<T, Error>;
pub type ID = u32;

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

        let light_config = self.handle_light_source();

        let keys = self.entity_manager.get_keys();
        for key in keys {
            let entity_view = self.entity_manager.as_ref_entity(key);
            let id = self.renderer.init_entity(&entity_view)?;

            //shader part
            self.handle_entity_transformation(&entity_view)?;
            self.handle_camera(key)?;
            self.renderer
                .update_default_shader_uniform_variables(&self.entity_manager.as_ref_entity(key))?;

            if let Some(light_config) = light_config.as_ref() {
                if entity_view.light.is_none() {
                    self.renderer
                        .update_light_uniform_variables(entity_view.entity_id, light_config)?;
                }
            }

            //final step to draw the entity
            self.renderer.draw_entity(id);
        }

        self.window.swap_buffers();
        Window::poll_events();
        Ok(())
    }

    fn handle_light_source(&self) -> Option<LightConfig> {
        if let Some(entity) = self.entity_manager.get_light_entity() {
            if let Some(camera) = self.camera.as_ref() {
                return calculate_light(&entity, camera.get_camera_vec_pos()).ok();
            }
        }

        None
    }

    fn handle_user_input_callbacks(&mut self) {
        event_interpreter::process_actions(self.event_manager.process_events(), self);
    }

    fn handle_entity_transformation(&self, entity: &View) -> Result<()> {
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
