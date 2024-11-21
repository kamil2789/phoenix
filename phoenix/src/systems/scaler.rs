use crate::{components::geometry::Radius, window::Resolution};

pub struct Scaler {
    resolution: Resolution,
}

impl Scaler {
    #[must_use]
    pub fn new(resolution: Resolution) -> Self {
        Self { resolution }
    }

    #[must_use]
    pub fn radius(&self, radius: f32) -> Radius {
        if self.resolution.width == self.resolution.height {
            return Radius {
                width: radius,
                height: radius,
            };
        }

        let mut width = radius;
        let mut height = radius;

        if self.resolution.width > self.resolution.height {
            width = f32::from(self.resolution.height) * radius / f32::from(self.resolution.width);
        } else {
            height = f32::from(self.resolution.width) * radius / f32::from(self.resolution.height);
        }

        Radius { width, height }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radius_width_bigger() {
        let scaler = Scaler::new(Resolution {
            width: 800,
            height: 600,
        });
        let radius = scaler.radius(100.0);
        assert!(radius.width > 74.999);
        assert!(radius.width < 75.001);

        assert!(radius.height > 99.999);
        assert!(radius.height < 100.001);
    }

    #[test]
    fn test_radius_height_bigger() {
        let scaler = Scaler::new(Resolution {
            width: 600,
            height: 800,
        });
        let radius = scaler.radius(100.0);
        assert!(radius.width > 99.999);
        assert!(radius.width < 100.001);

        assert!(radius.height > 74.999);
        assert!(radius.height < 75.001);
    }

    #[test]
    fn test_radius() {
        let scaler = Scaler::new(Resolution {
            width: 800,
            height: 800,
        });
        let radius = scaler.radius(100.0);
        assert!(radius.width > 99.999);
        assert!(radius.width < 100.001);

        assert!(radius.height > 99.999);
        assert!(radius.height < 100.001);
    }
}
