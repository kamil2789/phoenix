use cgmath::{vec3, InnerSpace, Matrix4, Vector3, Zero};

pub struct Transformer {
    translation: Vector3<f32>,
    rotation: Vector3<f32>,
    custom_axis_rotation_angle: Option<f32>,
    scale: Vector3<f32>,
}

pub struct Builder {
    result: Transformer,
}

impl Builder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            result: Transformer::default(),
        }
    }

    #[must_use]
    pub fn with_translation(mut self, translation: Vector3<f32>) -> Self {
        self.result.translation = translation;
        self
    }

    #[must_use]

    pub fn with_rotation(mut self, rotation: Vector3<f32>) -> Self {
        self.result.rotation = rotation;
        self
    }

    #[must_use]
    pub fn with_scale(mut self, scale: Vector3<f32>) -> Self {
        self.result.scale = scale;
        self
    }

    #[must_use]
    pub fn with_custom_axis_rotation_angle(mut self, axis: Vector3<f32>, angle: f32) -> Self {
        self.result.rotation = axis;
        self.result.custom_axis_rotation_angle = Some(angle);
        self
    }

    #[must_use]
    pub fn build(self) -> Transformer {
        self.result
    }
}

impl Transformer {
    #[must_use]
    pub fn new(translation: Vector3<f32>, rotation: Vector3<f32>, scale: Vector3<f32>) -> Self {
        Self {
            translation,
            rotation,
            scale,
            custom_axis_rotation_angle: None,
        }
    }

    #[must_use]
    pub fn new_translate(translation: Vector3<f32>) -> Self {
        Self {
            translation,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn new_rotate(rotation: Vector3<f32>) -> Self {
        Self {
            rotation,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn new_scale(scale: Vector3<f32>) -> Self {
        Self {
            scale,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn new_custom_axis_rotation_angle(axis: Vector3<f32>, angle: f32) -> Self {
        Self {
            rotation: axis,
            custom_axis_rotation_angle: Some(angle),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn get_translation_matrix(&self) -> Option<Matrix4<f32>> {
        if self.translation == Vector3::zero() {
            None
        } else {
            Some(Matrix4::<f32>::from_translation(self.translation))
        }
    }

    #[must_use]
    pub fn get_rotation_matrix(&self) -> Option<Matrix4<f32>> {
        if self.rotation == Vector3::zero() {
            return None;
        }

        if let Some(angle) = self.custom_axis_rotation_angle {
            Some(Matrix4::from_axis_angle(
                self.rotation.normalize(),
                cgmath::Deg(angle),
            ))
        } else {
            Some(
                Matrix4::<f32>::from_angle_x(cgmath::Deg(self.rotation.x))
                    * Matrix4::<f32>::from_angle_y(cgmath::Deg(self.rotation.y))
                    * Matrix4::<f32>::from_angle_z(cgmath::Deg(self.rotation.z)),
            )
        }
    }

    #[must_use]
    pub fn get_scale_matrix(&self) -> Option<Matrix4<f32>> {
        if self.scale == vec3(1.0, 1.0, 1.0) {
            None
        } else {
            Some(Matrix4::<f32>::from_nonuniform_scale(
                self.scale.x,
                self.scale.y,
                self.scale.z,
            ))
        }
    }
}

impl Default for Transformer {
    fn default() -> Self {
        Self {
            translation: Vector3::zero(),
            rotation: Vector3::zero(),
            scale: vec3(1.0, 1.0, 1.0),
            custom_axis_rotation_angle: None,
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use cgmath::{vec3, Matrix4, Vector3, Zero};

    #[test]
    fn test_new_transformer() {
        let result = super::Transformer::default();
        assert_eq!(result.translation, Vector3::zero());
        assert_eq!(result.rotation, Vector3::zero());
        assert_eq!(result.scale, vec3(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_new_builder() {
        let result = super::Builder::new();
        assert_eq!(result.result.translation, Vector3::zero());
        assert_eq!(result.result.rotation, Vector3::zero());
        assert_eq!(result.result.scale, vec3(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_builder() {
        let result = super::Builder::new()
            .with_translation(vec3(1.0, 2.0, 3.0))
            .with_rotation(vec3(4.0, 5.0, 6.0))
            .with_scale(vec3(7.0, 8.0, 9.0))
            .build();
        assert_eq!(result.translation, vec3(1.0, 2.0, 3.0));
        assert_eq!(result.rotation, vec3(4.0, 5.0, 6.0));
        assert_eq!(result.scale, vec3(7.0, 8.0, 9.0));
    }

    #[test]
    fn test_get_position_matrix() {
        let result = super::Transformer::new(
            vec3(1.0, 2.0, 3.0),
            vec3(4.0, 5.0, 6.0),
            vec3(7.0, 8.0, 9.0),
        )
        .get_translation_matrix();
        assert_eq!(
            result,
            Some(Matrix4::<f32>::from_translation(vec3(1.0, 2.0, 3.0)))
        );
    }

    #[test]
    fn test_get_scale_matrix() {
        let result = super::Transformer::new(
            vec3(1.0, 2.0, 3.0),
            vec3(4.0, 5.0, 6.0),
            vec3(7.0, 8.0, 9.0),
        )
        .get_scale_matrix();
        assert_eq!(
            result,
            Some(Matrix4::<f32>::from_nonuniform_scale(7.0, 8.0, 9.0))
        );
    }
}
