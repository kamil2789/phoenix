use super::{Point, Radius};
use crate::components::{Shape, ShapeType};

#[derive(Clone)]
pub struct Triangle {
    vertices: [f32; 9],
}

pub struct Circle {
    vertices: Vec<f32>,
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

impl Shape for Circle {
    fn get_vertices(&self) -> &[f32] {
        &self.vertices
    }

    fn get_type(&self) -> ShapeType {
        ShapeType::Circle
    }
}

impl Circle {
    #[must_use]
    pub fn new(center: &Point, radius: &Radius, mut segments: u8) -> Self {
        if segments < 5 {
            segments = 5;
        }

        let mut vertices = Vec::with_capacity(usize::from(segments) * 3 + 3);
        vertices.append(&mut center.clone().into());

        let angle = 360_f32 / f32::from(segments);
        let mut current_angle = angle;
        for _ in 0..segments {
            let x = radius.width * f32::sin(f32::to_radians(current_angle));
            let y = radius.height * f32::cos(f32::to_radians(current_angle));

            vertices.append(&mut vec![x + center.x, y + center.y, 0.0]);
            current_angle += angle;
        }

        Self { vertices }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::geometry::plane::{Shape, ShapeType, Triangle}; // Import the Shape trait and ShapeType enum.

    #[test]
    fn test_new_triangle() {
        let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
        let triangle = Triangle::new(vertices);

        assert_eq!(triangle.get_vertices(), &vertices);
        assert_eq!(triangle.get_type(), ShapeType::Triangle);
    }
}
