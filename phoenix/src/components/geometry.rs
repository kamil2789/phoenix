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
        value.clamp(0.0, 1.0)
    }
}

impl From<Point> for Vec<f32> {
    fn from(point: Point) -> Self {
        vec![point.x, point.y, point.z]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_into_vec() {
        let point = Point {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let vec: Vec<f32> = point.into();
        assert_eq!(vec, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_point_new_normalized() {
        let point = Point::new_normalized(-0.2, 0.5, 1.8);
        assert_eq!(point.x, 0.0);
        assert_eq!(point.y, 0.5);
        assert_eq!(point.z, 1.0);
    }
}
