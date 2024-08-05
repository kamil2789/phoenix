pub mod plane;
pub mod solid;

pub struct Radius {
    pub width: f32,
    pub height: f32,
}

#[derive(Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    #[must_use]
    pub fn new_normalized(x: f32, y: f32, z: f32) -> Self {
        Self {
            x: Self::normalize(x),
            y: Self::normalize(y),
            z: Self::normalize(z),
        }
    }

    fn normalize(value: f32) -> f32 {
        if value < 0.0 {
            0.0
        } else if value > 1.0 {
            1.0
        } else {
            value
        }
    }
}

impl From<Point> for Vec<f32> {
    fn from(point: Point) -> Self {
        vec![point.x, point.y, point.z]
    }
}
