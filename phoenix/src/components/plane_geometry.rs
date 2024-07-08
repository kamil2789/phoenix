use super::{Shape, ShapeType};

#[derive(Clone)]
pub struct Triangle {
    vertices: [f32; 9],
}

impl Triangle {
    #[must_use]
    pub fn new(vertices: [f32; 9]) -> Self {
        Self { vertices }
    }
}

impl Shape for Triangle {
    fn get_vertices(&self) -> &[f32] {
        &self.vertices
    }

    fn get_type(&self) -> ShapeType {
        ShapeType::Triangle
    }
}

#[cfg(test)]
mod tests {
    use crate::components::plane_geometry::{Shape, ShapeType, Triangle}; // Import the Shape trait and ShapeType enum.

    #[test]
    fn test_new_triangle() {
        let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
        let triangle = Triangle::new(vertices);

        assert_eq!(triangle.get_vertices(), &vertices);
        assert_eq!(triangle.get_type(), ShapeType::Triangle);
    }
}
