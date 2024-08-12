use crate::window::Resolution;
use cgmath::{perspective, Deg, InnerSpace, Matrix4, Point3, Vector3};

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
    is_first_mouse: bool,
    last_x: f32,
    last_y: f32,
    yaw: f32,
    pitch: f32,
    front: Vector3<f32>,
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
            speed: 2.5,
            position: Point3::new(0.0, 0.0, 0.0),
            is_first_mouse: true,
            last_x: f32::from(resolution.width) / 2.0,
            last_y: f32::from(resolution.height) / 2.0,
            yaw: -90.0,
            pitch: 0.0,
            front: Vector3::new(0.0, 0.0, -1.0),
        }
    }

    pub fn move_forward(&mut self, delta_time: f32) {
        self.position += self.speed * self.front * delta_time;
    }

    pub fn move_backward(&mut self,delta_time: f32) {
        self.position += -(self.speed * self.front * delta_time);
    }

    pub fn move_left(&mut self, delta_time: f32) {
        self.position += -(self.front.cross(CAMERA_UP).normalize() * self.speed) * delta_time;
    }

    pub fn move_right(&mut self, delta_time: f32) {
        self.position += self.front.cross(CAMERA_UP).normalize() * self.speed * delta_time;
    }

    pub fn change_fov(&mut self, yoffset: f32) {
        if self.field_of_vision >= 1.0 && self.field_of_vision <= 45.0 {
            self.field_of_vision -= yoffset;
        }
        self.field_of_vision = self.field_of_vision.clamp(1.0, 45.0);
    }

    pub fn change_orientation(&mut self, xpos: f32, ypos: f32) {
        if self.is_first_mouse {
            self.last_x = xpos;
            self.last_y = ypos;
            self.is_first_mouse = false;
        }

        let sensitivity: f32 = 0.1;
        let xoffset = (xpos - self.last_x) * sensitivity;
        let yoffset = (self.last_y - ypos) * sensitivity;
    
        self.last_x = xpos;
        self.last_y = ypos;

        self.yaw += xoffset;
        self.pitch = (self.pitch + yoffset).clamp(-89.0, 89.0);

        self.front = Vector3 {
            x: self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            y: self.pitch.to_radians().sin(),
            z: self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        }.normalize();
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
        Matrix4::look_at_rh(self.position, self.position + self.front, CAMERA_UP)
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
