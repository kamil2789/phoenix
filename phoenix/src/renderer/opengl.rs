use self::geometry_rendering::init_triangle;

use super::{Render, ID};
use crate::components::geometry::{Shape, ShapeType};
use crate::components::shaders::shader_program::ShaderProgram;
use crate::renderer::opengl::shader_compiler::compile;
use crate::renderer::Result;

mod geometry_rendering;
mod shader_compiler;

struct OpenGL {
    shaders_id: Vec<ID>,
}

impl Render for OpenGL {
    fn compile_shader_program(&mut self, shader_program: &ShaderProgram) -> Result<ID> {
        let shader_program_id = compile(
            shader_program.get_vertex_shader(),
            shader_program.get_fragment_shader(),
        )?;
        self.shaders_id.push(shader_program_id);
        Ok(shader_program_id)
    }
    /*
    fn init_shape(&mut self, shape: impl Shape) -> Result<ID> {
        match shape.get_type() {
            ShapeType::Triangle => init_triangle(vertices, color),
            _ => todo!(),
        }
    }
    */
}

impl Drop for OpenGL {
    fn drop(&mut self) {
        self.shaders_id.iter().for_each(|id| unsafe {
            gl::DeleteProgram(*id);
        })
    }
}
