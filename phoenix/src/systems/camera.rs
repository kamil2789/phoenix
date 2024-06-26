use crate::window::Resolution;
use cgmath::{perspective, Deg, Matrix4};

pub(super) struct Camera {
    //aspect_ratio: f32,
    //near_plane: f32,
    //far_plane: f32,
    //field_of_vision: f32,
    projection: Matrix4<f32>,
}

pub struct Config {
    pub near_plane: f32,
    pub far_plane: f32,
    pub field_of_vision: f32,
}

impl Camera {
    pub fn new(resolution: &Resolution, camera_config: &Config) -> Self {
        let aspect_ratio = f32::from(resolution.width) / f32::from(resolution.height);
        let projection = perspective(
            Deg(camera_config.field_of_vision),
            aspect_ratio,
            camera_config.near_plane,
            camera_config.far_plane,
        );
        Self {
            //aspect_ratio,
            //near_plane: camera_config.near_plane,
            //far_plane: camera_config.far_plane,
            //field_of_vision: camera_config.field_of_vision,
            projection,
        }
    }

    #[must_use]
    pub fn get_projection(&self) -> &Matrix4<f32> {
        &self.projection
    }
}
