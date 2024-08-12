use crate::window::Resolution;
use cgmath::{perspective, Deg, InnerSpace, Matrix4, Point3, Vector3};

// camera
const CAMERA_FRONT: Vector3<f32> = Vector3 {
    x: 0.0,
    y: 0.0,
    z: -1.0,
};
const CAMERA_UP: Vector3<f32> = Vector3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

pub(super) struct Camera {
    aspect_ratio: f32,
    near_plane: f32,
    far_plane: f32,
    field_of_vision: f32,
    speed: f32,
    position: Point3<f32>,
}

pub struct Config {
    pub near_plane: f32,
    pub far_plane: f32,
    pub field_of_vision: f32,
}

impl Camera {
    pub fn new(resolution: &Resolution, camera_config: &Config) -> Self {
        let aspect_ratio = f32::from(resolution.width) / f32::from(resolution.height);
        Self {
            aspect_ratio,
            near_plane: camera_config.near_plane,
            far_plane: camera_config.far_plane,
            field_of_vision: camera_config.field_of_vision,
            speed: 0.01,
            position: Point3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn move_forward(&mut self) {
        self.position += self.speed * CAMERA_FRONT;
    }

    pub fn move_backward(&mut self) {
        self.position += -(self.speed * CAMERA_FRONT);
    }

    pub fn move_left(&mut self) {
        self.position += -(CAMERA_FRONT.cross(CAMERA_UP).normalize() * self.speed);
    }

    pub fn move_right(&mut self) {
        self.position += CAMERA_FRONT.cross(CAMERA_UP).normalize() * self.speed;
    }

    pub fn change_fov(&mut self, yoffset: f32) {
        //dbg!(self.field_of_vision);
        if self.field_of_vision >= 1.0 && self.field_of_vision <= 45.0 {
            self.field_of_vision -= yoffset;
        }
        self.field_of_vision = self.field_of_vision.clamp(1.0, 45.0);
    }

    #[must_use]
    pub fn get_projection(&self) -> Matrix4<f32> {
        perspective(
            Deg(self.field_of_vision),
            self.aspect_ratio,
            self.near_plane,
            self.far_plane,
        )
    }

    pub fn get_camera_position(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(self.position, self.position + CAMERA_FRONT, CAMERA_UP)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            near_plane: 0.1,
            far_plane: 100.0,
            field_of_vision: 45.0,
        }
    }
}
