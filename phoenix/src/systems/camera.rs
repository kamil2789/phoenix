use crate::window::Resolution;
use cgmath::{perspective, Deg, InnerSpace, Matrix4, Point3, Vector3};

const CAMERA_UP: Vector3<f32> = Vector3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

pub(super) struct Camera {
    projection: Projection,
    movement: Movement,
    orientation: Orientation,
    mouse_input: MouseInput,
}

struct Projection {
    aspect_ratio: f32,
    near_plane: f32,
    far_plane: f32,
    field_of_vision: f32,
}

struct Movement {
    speed: f32,
    mouse_sensitivity: f32,
    position: Point3<f32>,
}

struct Orientation {
    yaw: f32,
    pitch: f32,
    front: Vector3<f32>,
}

struct MouseInput {
    is_first_mouse: bool,
    last_x: f32,
    last_y: f32,
}

pub struct Config {
    pub near_plane: f32,
    pub far_plane: f32,
    pub field_of_vision: f32,
    pub speed: f32,
    pub mouse_sensitivity: f32,
}

impl Camera {
    pub fn new(resolution: &Resolution, camera_config: &Config) -> Self {
        let aspect_ratio = f32::from(resolution.width) / f32::from(resolution.height);
        Self {
            projection: Projection {
                aspect_ratio,
                near_plane: camera_config.near_plane,
                far_plane: camera_config.far_plane,
                field_of_vision: camera_config.field_of_vision,
            },
            movement: Movement {
                speed: camera_config.speed,
                position: Point3::new(0.0, 0.0, 0.0),
                mouse_sensitivity: camera_config.mouse_sensitivity,
            },
            orientation: Orientation {
                yaw: -90.0,
                pitch: 0.0,
                front: Vector3::new(0.0, 0.0, -1.0),
            },
            mouse_input: MouseInput {
                is_first_mouse: true,
                last_x: f32::from(resolution.width) / 2.0,
                last_y: f32::from(resolution.height) / 2.0,
            },
        }
    }

    pub fn move_forward(&mut self, delta_time: f32) {
        self.movement.position += self.movement.speed * self.orientation.front * delta_time;
    }

    pub fn move_backward(&mut self, delta_time: f32) {
        self.movement.position += -(self.movement.speed * self.orientation.front * delta_time);
    }

    pub fn move_left(&mut self, delta_time: f32) {
        self.movement.position += -(self.orientation.front.cross(CAMERA_UP).normalize()
            * self.movement.speed)
            * delta_time;
    }

    pub fn move_right(&mut self, delta_time: f32) {
        self.movement.position +=
            self.orientation.front.cross(CAMERA_UP).normalize() * self.movement.speed * delta_time;
    }

    //yoffset is mostly a value of 1 or -1
    pub fn change_fov(&mut self, yoffset: f32) {
        self.projection.field_of_vision -= yoffset;
        self.projection.field_of_vision = self.projection.field_of_vision.clamp(1.0, 45.0);
    }

    pub fn change_orientation(&mut self, xpos: f32, ypos: f32) {
        if self.mouse_input.is_first_mouse {
            self.mouse_input.last_x = xpos;
            self.mouse_input.last_y = ypos;
            self.mouse_input.is_first_mouse = false;
        }

        let xoffset = (xpos - self.mouse_input.last_x) * self.movement.mouse_sensitivity;
        let yoffset = (self.mouse_input.last_y - ypos) * self.movement.mouse_sensitivity;

        self.mouse_input.last_x = xpos;
        self.mouse_input.last_y = ypos;

        self.orientation.yaw += xoffset;
        self.orientation.pitch = (self.orientation.pitch + yoffset).clamp(-89.0, 89.0);

        self.orientation.front = Vector3 {
            x: self.orientation.yaw.to_radians().cos() * self.orientation.pitch.to_radians().cos(),
            y: self.orientation.pitch.to_radians().sin(),
            z: self.orientation.yaw.to_radians().sin() * self.orientation.pitch.to_radians().cos(),
        }
        .normalize();
    }

    #[must_use]
    pub fn get_projection(&self) -> Matrix4<f32> {
        perspective(
            Deg(self.projection.field_of_vision),
            self.projection.aspect_ratio,
            self.projection.near_plane,
            self.projection.far_plane,
        )
    }

    pub fn get_camera_position(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(
            self.movement.position,
            self.movement.position + self.orientation.front,
            CAMERA_UP,
        )
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            near_plane: 0.1,
            far_plane: 100.0,
            field_of_vision: 45.0,
            speed: 2.5,
            mouse_sensitivity: 0.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use cgmath::assert_relative_eq;

    use super::*;

    fn create_camera() -> Camera {
        Camera::new(
            &Resolution {
                width: 800,
                height: 600,
            },
            &Config::default(),
        )
    }

    #[test]
    fn test_camera_move_forward() {
        //camera speed is set to 2.5
        let mut camera = create_camera();

        assert_relative_eq!(camera.movement.position, Point3::new(0.0, 0.0, 0.0));

        camera.move_forward(0.5);
        assert_relative_eq!(camera.movement.position, Point3::new(0.0, 0.0, -1.25));

        camera.move_forward(0.5);
        assert_relative_eq!(camera.movement.position, Point3::new(0.0, 0.0, -2.5));
    }

    #[test]
    fn test_camera_move_backward() {
        //camera speed is set to 2.5
        let mut camera = create_camera();

        assert_relative_eq!(camera.movement.position, Point3::new(0.0, 0.0, 0.0));

        camera.move_backward(0.5);
        assert_relative_eq!(camera.movement.position, Point3::new(0.0, 0.0, 1.25));

        camera.move_backward(0.5);
        assert_relative_eq!(camera.movement.position, Point3::new(0.0, 0.0, 2.5));
    }

    #[test]
    fn test_camera_move_right() {
        //camera speed is set to 2.5
        let mut camera = create_camera();

        assert_relative_eq!(camera.movement.position, Point3::new(0.0, 0.0, 0.0));

        camera.move_right(0.5);
        assert_relative_eq!(camera.movement.position, Point3::new(1.25, 0.0, 0.0));

        camera.move_right(0.5);
        assert_relative_eq!(camera.movement.position, Point3::new(2.5, 0.0, 0.0));
    }

    #[test]
    fn test_camera_move_left() {
        //camera speed is set to 2.5
        let mut camera = create_camera();

        assert_relative_eq!(camera.movement.position, Point3::new(0.0, 0.0, 0.0));

        camera.move_left(0.5);
        assert_relative_eq!(camera.movement.position, Point3::new(-1.25, 0.0, 0.0));

        camera.move_left(0.5);
        assert_relative_eq!(camera.movement.position, Point3::new(-2.5, 0.0, 0.0));
    }

    #[test]
    fn test_camera_change_field_of_vision() {
        let mut camera = create_camera();

        assert_relative_eq!(camera.projection.field_of_vision, 45.0);

        camera.change_fov(2.0);
        assert_relative_eq!(camera.projection.field_of_vision, 43.0);

        camera.change_fov(-1.0);
        assert_relative_eq!(camera.projection.field_of_vision, 44.0);
    }

    #[test]
    fn test_camera_change_field_of_vision_check_min_value() {
        let mut camera = create_camera();

        assert_relative_eq!(camera.projection.field_of_vision, 45.0);

        camera.change_fov(50.0);
        assert_relative_eq!(camera.projection.field_of_vision, 1.0);
    }

    #[test]
    fn test_camera_change_field_of_vision_check_max_value() {
        let mut camera = create_camera();

        assert_relative_eq!(camera.projection.field_of_vision, 45.0);

        camera.change_fov(-5.0);
        assert_relative_eq!(camera.projection.field_of_vision, 45.0);
    }

    #[test]
    fn test_camera_change_orientation() {
        //Resolition is 800x600
        let mut camera = create_camera();

        assert_eq!(camera.mouse_input.is_first_mouse, true);
        assert_relative_eq!(camera.mouse_input.last_x, 400.0);
        assert_relative_eq!(camera.mouse_input.last_y, 300.0);

        assert_relative_eq!(camera.orientation.yaw, -90.0);
        assert_relative_eq!(camera.orientation.pitch, 0.0);
        assert_relative_eq!(camera.orientation.front, Vector3::new(0.0, 0.0, -1.0));

        //arbitrary values
        camera.change_orientation(1200.0, 1000.0);

        assert_eq!(camera.mouse_input.is_first_mouse, false);
        assert_relative_eq!(camera.mouse_input.last_x, 1200.0);
        assert_relative_eq!(camera.mouse_input.last_y, 1000.0);

        assert_relative_eq!(camera.orientation.yaw, -90.0);
        assert_relative_eq!(camera.orientation.pitch, 0.0);
        assert_relative_eq!(camera.orientation.front.z, -1.0);
        assert!(camera.orientation.front.x < 0.0);
        assert_relative_eq!(camera.orientation.front.y, 0.0);
    }

    #[test]
    fn test_camera_get_projection() {
        let camera = create_camera();

        assert_relative_eq!(camera.projection.aspect_ratio, 800.0 / 600.0);
        assert_relative_eq!(camera.projection.field_of_vision, 45.0);
        assert_relative_eq!(camera.projection.near_plane, 0.1);
        assert_relative_eq!(camera.projection.far_plane, 100.0);

        let projection = camera.get_projection();

        assert_relative_eq!(
            projection,
            Matrix4::<f32>::new(
                1.81066, 0.0, 0.0, 0.0, 0.0, 2.4142134, 0.0, 0.0, 0.0, 0.0, -1.002002, -1.0, 0.0,
                0.0, -0.2002002, 0.0
            )
        );
    }

    #[test]
    fn test_camera_get_camera_position() {
        let camera = create_camera();

        assert_relative_eq!(camera.movement.position, Point3::new(0.0, 0.0, 0.0));
        assert_relative_eq!(camera.orientation.front, Vector3::new(0.0, 0.0, -1.0));

        let position = camera.get_camera_position();

        assert_relative_eq!(
            position,
            Matrix4::<f32>::new(
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0
            )
        );
    }
}
