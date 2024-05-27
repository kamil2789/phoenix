use std::collections::HashMap;

use crate::components::{
    color::RGBA, geometry::Shape, shaders::shader_program::ShaderProgram, Component,
};
pub type ID = u32;

pub struct Entity {
    components: Vec<Component>,
}

pub struct SceneManager {
    colors: HashMap<ID, RGBA>,
    shapes: HashMap<ID, Box<dyn Shape>>,
    shader_programs: HashMap<ID, ShaderProgram>,
}
