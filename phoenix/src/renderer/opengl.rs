use std::collections::HashMap;

use super::{Render, ID};
use crate::components::color::RGBA;
use crate::components::geometry::ShapeType;
use crate::components::shaders::shader_program::ShaderProgram;
use crate::managers::entity::View;
use crate::renderer::Result;

mod geometry_rendering;
mod shader_compiler;

pub type ShaderID = u32;
pub type BufferID = u32;

#[derive(Default, Clone)]
pub struct OpenGL {
    shaders_id: HashMap<BufferID, ShaderID>,
    buffers: HashMap<BufferID, Buffers>,
}

#[derive(Clone)]
struct Buffers {
    pub vertex_array_object: u32,
    pub vertex_buffer_object: u32,
}

impl Buffers {
    fn new(vertex_array_object: u32, vertex_buffer_object: u32) -> Buffers {
        Buffers {
            vertex_array_object,
            vertex_buffer_object,
        }
    }
}

impl Render for OpenGL {
    fn compile_shader_program(&mut self, shader_program: &ShaderProgram) -> Result<ID> {
        let shader_program_id = shader_compiler::compile(
            shader_program.get_vertex_shader(),
            shader_program.get_fragment_shader(),
        )?;
        Ok(shader_program_id)
    }

    fn set_background_color(&self, color: &RGBA) {
        unsafe {
            let rgba = color.get_as_normalized_f32();
            gl::ClearColor(rgba[0], rgba[1], rgba[2], rgba[3]);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    fn init_entity(&mut self, entity: View) -> Result<ID> {
        if self.buffers.contains_key(&entity.entity_id) {
            return Ok(entity.entity_id); //already initialized
        }

        //color not supported
        if let Some(value) = entity.shape {
            match value.get_type() {
                ShapeType::Triangle => {
                    let buffers = geometry_rendering::init_triangle(value.get_vertices());
                    self.buffers.insert(entity.entity_id, buffers);
                }
            }
        }

        if let Some(value) = entity.shader_program {
            let id = self.compile_shader_program(value)?;
            self.shaders_id.insert(entity.entity_id, id);
        }

        Ok(entity.entity_id)
    }

    fn draw_entity(&self, entity_id: ID) {
        //support only triangles
        unsafe {
            if let Some(shader) = self.shaders_id.get(&entity_id) {
                gl::UseProgram(*shader);
            }

            if let Some(triangle) = self.buffers.get(&entity_id) {
                gl::BindVertexArray(triangle.vertex_array_object);
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
            }
        }
    }
}

impl Drop for OpenGL {
    fn drop(&mut self) {
        self.shaders_id.iter().for_each(|(_, id)| unsafe {
            gl::DeleteProgram(*id);
        });
    }
}
