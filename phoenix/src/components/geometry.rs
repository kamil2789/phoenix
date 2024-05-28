pub trait Shape {
    fn get_vertices(&self) -> &[f32];
    fn get_type(&self) -> ShapeType;
}

pub enum ShapeType {
    Triangle,
}

pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Triangle {
    vertices: [f32; 9],
}

impl Triangle {
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
