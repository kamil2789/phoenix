use self::{color::RGBA, geometry::Triangle, shaders::shader_program::ShaderProgram};

pub mod color;
pub mod geometry;
pub mod shaders;

pub enum Component {
    Color(RGBA),
    Geometry(Triangle),
    ShaderProgram(ShaderProgram),
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
