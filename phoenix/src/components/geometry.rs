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
