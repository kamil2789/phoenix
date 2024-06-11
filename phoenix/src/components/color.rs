#[derive(Clone, Debug, PartialEq)]
pub struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: f32,
}

impl RGBA {
    #[must_use]
    pub fn new(red: u8, green: u8, blue: u8, alpha: f32) -> Self {
        RGBA {
            r: red,
            g: green,
            b: blue,
            a: RGBA::normalized_alpha(alpha),
        }
    }

    #[must_use]
    pub fn empty() -> Self {
        RGBA {
            r: 0,
            g: 0,
            b: 0,
            a: 1_f32,
        }
    }

    #[must_use]
    pub fn from_hex(color: u32) -> Self {
        let bytes = color.to_be_bytes();
        RGBA {
            r: bytes[0],
            g: bytes[1],
            b: bytes[2],
            a: RGBA::convert_from_u8_to_normalized_f32(bytes[3]),
        }
    }

    #[must_use]
    pub fn get_as_normalized_f32(&self) -> [f32; 4] {
        let red = RGBA::convert_from_u8_to_normalized_f32(self.r);
        let green = RGBA::convert_from_u8_to_normalized_f32(self.g);
        let blue = RGBA::convert_from_u8_to_normalized_f32(self.b);
        let alpha = self.a;
        [red, green, blue, alpha]
    }

    #[must_use]
    pub fn get_rgba(&self) -> (u8, u8, u8, f32) {
        (self.r, self.g, self.b, self.a)
    }

    #[must_use]
    fn convert_from_u8_to_normalized_f32(number: u8) -> f32 {
        (1_f32 / f32::from(u8::MAX)) * f32::from(number)
    }

    #[must_use]
    fn normalized_alpha(number: f32) -> f32 {
        if number > 1_f32 {
            return 1_f32;
        }

        if number < 0_f32 {
            return 0_f32;
        }

        number
    }
}

impl Default for RGBA {
    fn default() -> Self {
        RGBA::from_hex(0xFF_FF_FF_FF)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_color_rgba() {
        let red = RGBA::new(255, 0, 0, 1_f32);
        assert_eq!((255, 0, 0, 1_f32), red.get_rgba());
    }

    #[test]
    fn test_new_color_empty() {
        let color = RGBA::empty();
        assert_eq!((0, 0, 0, 1_f32), color.get_rgba());
    }

    #[test]
    fn test_new_color_default() {
        let color = RGBA::default();
        assert_eq!((255, 255, 255, 1_f32), color.get_rgba());
    }

    #[test]
    fn test_new_rgba_alpha_not_in_scope() {
        let oversized = RGBA::new(255, 0, 0, 23_f32);
        assert_eq!((255, 0, 0, 1_f32), oversized.get_rgba());

        let minus_value = RGBA::new(255, 0, 0, -0.4_f32);
        assert_eq!((255, 0, 0, 0_f32), minus_value.get_rgba());
    }

    #[test]
    fn test_new_color_rgba_from_hex() {
        let color = RGBA::from_hex(0xff00ffff);
        assert_eq!((255, 0, 255, 1_f32), color.get_rgba());
    }

    #[test]
    fn test_get_as_normalized_f32() {
        let color = RGBA::new(255, 0, 0, 1_f32);
        let rgba = color.get_as_normalized_f32();
        assert_eq!([1_f32, 0.0, 0.0, 1_f32], rgba);
    }
}
