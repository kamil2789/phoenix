use cgmath::{InnerSpace, Matrix4, SquareMatrix, Vector3};

pub struct Transformer {
    matrix: Matrix4<f32>,
}

pub struct Builder {
    translation: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: Vector3<f32>,
    custom_axis_rotation_angle: Option<f32>,
}

impl Builder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            translation: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
            custom_axis_rotation_angle: None,
        }
    }

    #[must_use]
    pub fn with_translation(mut self, translation: Vector3<f32>) -> Self {
        self.translation = translation;
        self
    }

    #[must_use]

    pub fn with_rotation(mut self, rotation: Vector3<f32>) -> Self {
        self.rotation = rotation;
        self
    }

    #[must_use]
    pub fn with_scale(mut self, scale: Vector3<f32>) -> Self {
        self.scale = scale;
        self
    }

    #[must_use]
    pub fn with_custom_axis_rotation_angle(mut self, axis: Vector3<f32>, angle: f32) -> Self {
        self.custom_axis_rotation_angle = Some(angle);
        self.rotation = axis;
        self
    }

    #[must_use]
    pub fn build(self) -> Transformer {
        if let Some(angle) = self.custom_axis_rotation_angle {
            Transformer::new_with_custom_axis_rotation(
                self.translation,
                self.rotation,
                angle,
                self.scale,
            )
        } else {
            Transformer::new(self.translation, self.rotation, self.scale)
        }
    }
}

impl Transformer {
    #[must_use]
    pub fn new(translation: Vector3<f32>, rotation: Vector3<f32>, scale: Vector3<f32>) -> Self {
        let matrix = Self::apply_translation(&translation)
            * Self::apply_rotation(&rotation, None)
            * Self::apply_scale(&scale);
        Self { matrix }
    }

    #[must_use]
    pub fn new_with_custom_axis_rotation(
        translation: Vector3<f32>,
        rotation: Vector3<f32>,
        angle: f32,
        scale: Vector3<f32>,
    ) -> Self {
        let matrix = Self::apply_translation(&translation)
            * Self::apply_rotation(&rotation, Some(angle))
            * Self::apply_scale(&scale);
        Self { matrix }
    }

    #[must_use]
    pub fn new_translate(translation: Vector3<f32>) -> Self {
        Self {
            matrix: Self::apply_translation(&translation),
        }
    }

    #[must_use]
    pub fn new_rotate(rotation: Vector3<f32>) -> Self {
        Self {
            matrix: Self::apply_rotation(&rotation, None),
        }
    }

    #[must_use]
    pub fn new_scale(scale: Vector3<f32>) -> Self {
        Self {
            matrix: Self::apply_scale(&scale),
        }
    }

    #[must_use]
    pub fn new_custom_axis_rotation_angle(axis: Vector3<f32>, angle: f32) -> Self {
        Self {
            matrix: Self::apply_rotation(&axis, Some(angle)),
        }
    }

    #[must_use]
    pub fn get_matrix(&self) -> Matrix4<f32> {
        self.matrix
    }

    #[must_use]
    fn apply_translation(translation: &Vector3<f32>) -> Matrix4<f32> {
        Matrix4::<f32>::from_translation(*translation)
    }

    #[must_use]
    fn apply_rotation(
        rotation: &Vector3<f32>,
        custom_axis_rotation_angle: Option<f32>,
    ) -> Matrix4<f32> {
        if let Some(angle) = custom_axis_rotation_angle {
            Matrix4::from_axis_angle(rotation.normalize(), cgmath::Deg(angle))
        } else {
            Matrix4::<f32>::from_angle_x(cgmath::Deg(rotation.x))
                * Matrix4::<f32>::from_angle_y(cgmath::Deg(rotation.y))
                * Matrix4::<f32>::from_angle_z(cgmath::Deg(rotation.z))
        }
    }

    #[must_use]
    fn apply_scale(scale: &Vector3<f32>) -> Matrix4<f32> {
        Matrix4::<f32>::from_nonuniform_scale(scale.x, scale.y, scale.z)
    }
}

impl Default for Transformer {
    fn default() -> Self {
        Self {
            matrix: Matrix4::identity(),
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
    use super::{Builder, Transformer};
    use cgmath::{assert_relative_eq, vec3, Matrix4, SquareMatrix, Vector3};

    #[test]
    fn test_new_transformer() {
        let result = Transformer::default();
        assert_eq!(result.matrix, Matrix4::identity());
    }

    #[test]
    fn test_new_builder() {
        let builder = Builder::new();
        assert_eq!(builder.translation, Vector3::new(0.0, 0.0, 0.0));
        assert_eq!(builder.rotation, Vector3::new(0.0, 0.0, 0.0));
        assert_eq!(builder.scale, Vector3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_new_builder_with_custom_axis_rotation_angle() {
        let transformer = Builder::new()
            .with_custom_axis_rotation_angle(vec3(1.0, 1.5, 3.0), 4.0)
            .build();
        let result = transformer.get_matrix();
        assert_relative_eq!(
            result,
            Matrix4::<f32>::new(
                0.9977629,
                0.060089543,
                -0.02929908,
                0.0,
                -0.05949299,
                0.99801147,
                0.02082525,
                0.0,
                0.030492187,
                -0.01903559,
                0.9993537,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0
            )
        );
    }

    #[test]
    fn test_builder() {
        let result = Builder::new()
            .with_translation(vec3(1.0, 2.0, 3.0))
            .with_rotation(vec3(4.0, 5.0, 6.0))
            .with_scale(vec3(7.0, 8.0, 9.0))
            .build();
        let matrix = Transformer::new(
            vec3(1.0, 2.0, 3.0),
            vec3(4.0, 5.0, 6.0),
            vec3(7.0, 8.0, 9.0),
        )
        .get_matrix();
        assert_relative_eq!(result.get_matrix(), matrix);
    }

    #[test]
    fn test_transfomer_translate() {
        let result = Transformer::new(
            vec3(1.0, 2.0, 3.0),
            vec3(0.0, 0.0, 0.0),
            vec3(1.0, 1.0, 1.0),
        )
        .get_matrix();
        assert_eq!(
            result,
            Matrix4::<f32>::from_translation(vec3(1.0, 2.0, 3.0))
        );
    }

    #[test]
    fn test_transformer_scale() {
        let result = Transformer::new(
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 0.0, 0.0),
            vec3(7.0, 8.0, 9.0),
        );

        assert_eq!(
            result.get_matrix(),
            Matrix4::new(
                7.0, 0.0, 0.0, 0.0, 0.0, 8.0, 0.0, 0.0, 0.0, 0.0, 9.0, 0.0, 0.0, 0.0, 0.0, 1.0
            )
        );
    }

    #[test]
    fn test_transformer_new_translate() {
        let result = Transformer::new_translate(vec3(1.0, 2.0, 5.0));

        assert_relative_eq!(
            result.get_matrix(),
            Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 2.0, 5.0, 1.0
            )
        );
    }

    #[test]
    fn test_transformer_new_scale() {
        let result = Transformer::new_scale(vec3(1.0, 2.0, 2.0));

        assert_relative_eq!(
            result.get_matrix(),
            Matrix4::new(
                1.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 1.0
            )
        );
    }

    #[test]
    fn test_transformer_new_rotate() {
        let result = Transformer::new_rotate(vec3(1.0, 0.0, 0.0));

        assert_relative_eq!(
            result.get_matrix(),
            Matrix4::new(
                1.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.9998477,
                0.017452406,
                0.0,
                0.0,
                -0.017452406,
                0.9998477,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0
            )
        );
    }
}
