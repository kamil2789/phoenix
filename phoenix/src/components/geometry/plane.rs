use super::{Point, Radius};
use crate::components::{FillMode, Shape, ShapeType};

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

    fn get_fill_mode(&self) -> FillMode {
        FillMode::Fan
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
    use crate::components::{
        geometry::{
            plane::{Circle, Shape, ShapeType, Triangle},
            Point, Radius,
        },
        FillMode,
    }; // Import the Shape trait and ShapeType enum.

    #[test]
    fn test_new_triangle() {
        let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
        let triangle = Triangle::new(vertices);

        assert_eq!(triangle.get_vertices(), &vertices);
        assert_eq!(triangle.get_type(), ShapeType::Triangle);
    }

    #[test]
    fn test_new_circle() {
        let center = Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let radius = Radius {
            width: 0.4,
            height: 0.4,
        };
        let circle = Circle::new(&center, &radius, 6);
        assert_eq!(circle.get_vertices().len(), 21);
        assert_eq!(circle.get_type(), ShapeType::Circle);
        assert_eq!(circle.get_fill_mode(), FillMode::Fan);
    }

    #[test]
    fn test_new_circle_with_two_segments() {
        let center = Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let radius = Radius {
            width: 0.4,
            height: 0.4,
        };
        let circle = Circle::new(&center, &radius, 2);
        assert_eq!(circle.get_vertices().len(), 18);
        assert_eq!(circle.get_type(), ShapeType::Circle);
        assert_eq!(circle.get_fill_mode(), FillMode::Fan);
    }
}
