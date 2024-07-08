use super::{Shape, ShapeType};

pub struct Cube {
    vertices: [f32; 108],
}

impl Cube {
    #[must_use]
    pub fn new(side: f32, center_point: [f32; 3]) -> Self {
        let half_side = side / 2.0;
        let [cx, cy, cz] = center_point;

        let left = cx - half_side;
        let right = cx + half_side;
        let bottom = cy - half_side;
        let top = cy + half_side;
        let far = cz - half_side;
        let near = cz + half_side;

        let vertices = [
            left, bottom, far, right, bottom, far, right, top, far, right, top, far, left, top,
            far, left, bottom, far, left, bottom, near, right, bottom, near, right, top, near,
            right, top, near, left, top, near, left, bottom, near, left, top, near, left, top, far,
            left, bottom, far, left, bottom, far, left, bottom, near, left, top, near, right, top,
            near, right, top, far, right, bottom, far, right, bottom, far, right, bottom, near,
            right, top, near, left, bottom, far, right, bottom, far, right, bottom, near, right,
            bottom, near, left, bottom, near, left, bottom, far, left, top, far, right, top, far,
            right, top, near, right, top, near, left, top, near, left, top, far,
        ];

        Self { vertices }
    }
}

impl Shape for Cube {
    fn get_vertices(&self) -> &[f32] {
        &self.vertices
    }

    fn get_type(&self) -> ShapeType {
        ShapeType::Cube
    }
}
