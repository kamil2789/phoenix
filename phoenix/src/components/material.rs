use cgmath::Vector3;

pub struct Material {
    pub ambient: Vector3<f32>,
    pub diffuse: Vector3<f32>,
    pub specular: Vector3<f32>,
    pub shininess: f32,
}

impl Material {
    #[must_use]
    pub fn new_shininess(shininess: f32) -> Self {
        Self {
            shininess,
            ..Default::default()
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient: Vector3::new(0.2, 0.2, 0.2),
            diffuse: Vector3::new(0.8, 0.8, 0.8),
            specular: Vector3::new(0.5, 0.5, 0.5),
            shininess: 32.0,
        }
    }
}
