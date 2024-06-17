use color::Color;

use self::{geometry::Shape, shaders::ShaderSource};

pub mod color;
pub mod geometry;
pub mod shaders;
pub mod texture;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("File could not be opened, path: {0}")]
    SourceFileError(String),
    #[error("Error when loading data from a file ")]
    ReadFileError(#[from] std::io::Error),
    #[error("Error when loading texture data")]
    ImageError(#[from] image::ImageError),
}

pub enum Component {
    Color(Color),
    Geometry(Box<dyn Shape>),
    ShaderProgram(ShaderSource),
    Texture(texture::Texture),
}

#[cfg(test)]
mod tests {
    use crate::components::Component;

    use std::mem;
    const MEMORY_USAGE_FOR_COMPONENTS_ENUM: usize = 48;

    #[test]
    fn test_check_maximum_memory_usage_for_components_enum() {
        assert_eq!(
            MEMORY_USAGE_FOR_COMPONENTS_ENUM,
            mem::size_of::<Component>()
        );
    }
}
