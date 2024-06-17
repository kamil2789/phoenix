use image::{self, DynamicImage};
use std::rc::Rc;

pub type TexID = u32;

#[derive(Clone)]
pub struct Texture {
    data: Rc<DynamicImage>,
    config: Config,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Config {
    pub wrapping_horizontal: Wrapping,
    pub wrapping_vertical: Wrapping,
    pub min_filtering: MinFiltering,
    pub max_filtering: Filtering,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Wrapping {
    Repeat,
    MirroredRepeat,
    ClampToEdge,
    ClampToBorder,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Filtering {
    Linear,
    Nearest,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Mipmaps {
    NearestMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapNearest,
    LinearMipmapLinear,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MinMag {
    Minifying,
    Magnifying,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MinFiltering {
    Mimpmap(Mipmaps),
    Filtering(Filtering),
}

impl Texture {
    #[must_use]
    pub fn new(data: Rc<DynamicImage>, config: Config) -> Self {
        Self { data, config }
    }

    pub fn set_config(&mut self, config: Config) {
        self.config = config;
    }

    #[must_use]
    pub fn get_config(&self) -> &Config {
        &self.config
    }

    #[must_use]
    pub fn get_width(&self) -> u32 {
        self.data.width()
    }

    #[must_use]
    pub fn get_height(&self) -> u32 {
        self.data.height()
    }

    #[must_use]
    pub fn get_raw_data(&self) -> &[u8] {
        self.data.as_bytes()
    }
}

impl Default for Texture {
    fn default() -> Self {
        Self {
            data: Rc::new(DynamicImage::default()),
            config: Config {
                wrapping_horizontal: Wrapping::Repeat,
                wrapping_vertical: Wrapping::Repeat,
                min_filtering: MinFiltering::Filtering(Filtering::Linear),
                max_filtering: Filtering::Linear,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, Rgba};
    use std::rc::Rc;

    #[test]
    fn test_texture_creation() {
        let img =
            DynamicImage::ImageRgba8(image::ImageBuffer::from_pixel(2, 2, Rgba([0, 0, 0, 0])));
        let config = Config {
            wrapping_horizontal: Wrapping::Repeat,
            wrapping_vertical: Wrapping::Repeat,
            min_filtering: MinFiltering::Filtering(Filtering::Linear),
            max_filtering: Filtering::Linear,
        };
        let texture = Texture::new(Rc::new(img), config.clone());

        assert_eq!(texture.get_width(), 2);
        assert_eq!(texture.get_height(), 2);
        assert_eq!(texture.get_config(), &config);
    }

    #[test]
    fn test_set_config() {
        let img =
            DynamicImage::ImageRgba8(image::ImageBuffer::from_pixel(2, 2, Rgba([0, 0, 0, 0])));
        let config1 = Config {
            wrapping_horizontal: Wrapping::Repeat,
            wrapping_vertical: Wrapping::Repeat,
            min_filtering: MinFiltering::Filtering(Filtering::Linear),
            max_filtering: Filtering::Linear,
        };
        let config2 = Config {
            wrapping_horizontal: Wrapping::ClampToEdge,
            wrapping_vertical: Wrapping::ClampToEdge,
            min_filtering: MinFiltering::Mimpmap(Mipmaps::LinearMipmapLinear),
            max_filtering: Filtering::Nearest,
        };

        let mut texture = Texture::new(Rc::new(img), config1.clone());
        assert_eq!(texture.get_config(), &config1);

        texture.set_config(config2.clone());
        assert_eq!(texture.get_config(), &config2);
    }

    #[test]
    fn test_default_texture() {
        let default_texture = Texture::default();

        assert_eq!(default_texture.get_width(), 0);
        assert_eq!(default_texture.get_height(), 0);
        assert_eq!(
            default_texture.get_config().wrapping_horizontal,
            Wrapping::Repeat
        );
        assert_eq!(
            default_texture.get_config().wrapping_vertical,
            Wrapping::Repeat
        );
        assert!(matches!(
            default_texture.get_config().min_filtering,
            MinFiltering::Filtering(Filtering::Linear)
        ));
        assert_eq!(
            default_texture.get_config().max_filtering,
            Filtering::Linear
        );
    }

    #[test]
    fn test_get_raw_data() {
        let img =
            DynamicImage::ImageRgba8(image::ImageBuffer::from_pixel(2, 2, Rgba([255, 0, 0, 255])));
        let config = Config {
            wrapping_horizontal: Wrapping::Repeat,
            wrapping_vertical: Wrapping::Repeat,
            min_filtering: MinFiltering::Filtering(Filtering::Linear),
            max_filtering: Filtering::Linear,
        };
        let texture = Texture::new(Rc::new(img), config);

        let raw_data = texture.get_raw_data();
        let expected_data = vec![
            255, 0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255,
        ];
        assert_eq!(raw_data, expected_data.as_slice());
    }
}
