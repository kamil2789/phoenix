use cgmath::Vector3;

#[derive(Clone)]
pub struct Light {
    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
}

impl Default for Light {
    fn default() -> Self {
        Self {
            ambient: Vector3::new(0.2, 0.2, 0.2),
            diffuse: Vector3::new(0.8, 0.8, 0.8),
            specular: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}
