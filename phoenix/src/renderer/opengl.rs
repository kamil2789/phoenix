use std::collections::HashMap;
use std::rc::Rc;

use geometry_rendering::set_uniform_color;

use super::{Error, Render, ID};
use crate::components::color::{Color, RGBA};
use crate::components::geometry::{Shape, ShapeType};
use crate::components::shaders::ShaderSource;
use crate::components::texture::Texture;
use crate::entities::entity::View;
use crate::renderer::Result;

mod geometry_rendering;
mod shader_compiler;
mod textures;

pub type ShaderID = u32;
pub type TextureID = u32;
pub type EntityID = u32;

#[derive(Default, Clone)]
pub struct OpenGL {
    shaders_id: HashMap<EntityID, ShaderID>,
    compiled_shaders: HashMap<Rc<ShaderSource>, ShaderID>,
    buffers: HashMap<EntityID, Buffers>,
    textures: HashMap<EntityID, TextureID>,
}

#[derive(Clone, Default)]
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
    fn compile_shader_program(&mut self, shader_program: Rc<ShaderSource>) -> Result<ID> {
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

    /// Return the ID of the initialized entity
    fn init_entity(&mut self, entity: View) -> Result<ID> {
        if self.buffers.contains_key(&entity.entity_id) {
            return Ok(entity.entity_id); //already initialized
        }

        if let Some(shape) = entity.shape {
            let buffers = OpenGL::handle_vertices(shape, &entity)?;
            self.buffers.insert(entity.entity_id, buffers);
        }

        if let Some(value) = entity.shader_src {
            let id = self.handle_shader(value)?;
            self.shaders_id.insert(entity.entity_id, id);
        }

        if let Some(texture) = entity.texture {
            let texture_id = self.init_texture(texture)?;
            self.textures.insert(entity.entity_id, texture_id);
        }

        if entity.texture.is_some()
            && entity.color.is_some()
            && entity.color.unwrap().as_ref_uniform().is_some()
        {
            if let Some(shader_id) = self.shaders_id.get(&entity.entity_id) {
                geometry_rendering::set_uniform_bool("isUniformColor", *shader_id)?;
            };
        }

        if let Some(value) = entity.color {
            let shader_id = self.shaders_id.get(&entity.entity_id).unwrap_or(&0);
            if value.as_ref_uniform().is_some() {
                set_uniform_color("color", value, *shader_id)?;
            }
        }

        Ok(entity.entity_id)
    }

    fn draw_entity(&self, entity_id: ID) {
        //support only triangles
        unsafe {
            if let Some(shader) = self.shaders_id.get(&entity_id) {
                gl::UseProgram(*shader);
            }

            if let Some(texture) = self.textures.get(&entity_id) {
                gl::BindTexture(gl::TEXTURE_2D, *texture);
            }

            if let Some(triangle) = self.buffers.get(&entity_id) {
                gl::BindVertexArray(triangle.vertex_array_object);
                gl::DrawArrays(gl::TRIANGLES, 0, 3);
            }
        }
    }

    fn init_texture(&mut self, texture: &Texture) -> Result<ID> {
        textures::init_texture(texture)
    }
}

impl OpenGL {
    fn handle_shader(&mut self, shader: Rc<ShaderSource>) -> Result<ShaderID> {
        if let Some(val) = self.compiled_shaders.get(&shader) {
            return Ok(*val); //already compiled
        }

        let result = self.compile_shader_program(shader.clone())?;
        self.compiled_shaders.insert(shader, result);
        Ok(result)
    }

    fn handle_vertices(shape: &dyn Shape, entity: &View) -> Result<Buffers> {
        match shape.get_type() {
            ShapeType::Triangle => {
                if entity.texture.is_some() {
                    Ok(geometry_rendering::init_triangle_with_texture(
                        shape.get_vertices(),
                    ))
                } else if let Some(val_color) = entity.color {
                    OpenGL::handle_colored_triangle(shape, val_color)
                } else {
                    Ok(geometry_rendering::init_triangle(shape.get_vertices()))
                }
            }
        }
    }

    fn handle_colored_triangle(shape: &dyn Shape, color: &Color) -> Result<Buffers> {
        if color.as_ref_uniform().is_some() {
            Ok(geometry_rendering::init_triangle(shape.get_vertices()))
        } else if let Some(vert_color) = color.as_ref_vertices() {
            Ok(geometry_rendering::init_triangle_with_color(
                shape.get_vertices(),
                vert_color,
            ))
        } else {
            Err(Error::RenderingError(
                "Not supported color type".to_string(),
            ))
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

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::OpenGL;
    use crate::components::color::Color;
    use crate::window::{GlfwConfig, Resolution};
    use crate::{
        components::{geometry::Triangle, shaders::ShaderSource},
        entities::entity::View,
        renderer::{
            shaders::{UNIFORM_TRIANGLE_FRAG, UNIFORM_TRIANGLE_VERT},
            Render,
        },
    };
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_opengl_init_entity() {
        let config = GlfwConfig::create().unwrap();
        let window = config
            .create_window("test_opengl_init_entity", Resolution::default())
            .unwrap();
        window.set_current().unwrap();

        let color = Color::default();
        let vertices = Triangle::new([0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
        let shader = Rc::new(ShaderSource::new(
            UNIFORM_TRIANGLE_VERT,
            UNIFORM_TRIANGLE_FRAG,
        ));
        let entity = View::new(1, Some(&color), Some(&vertices), Some(shader), None);

        let mut renderer = OpenGL::default();
        let ret = renderer.init_entity(entity);
        assert!(ret.is_ok());
        assert_eq!(renderer.compiled_shaders.len(), 1);
        assert_eq!(renderer.buffers.len(), 1);
        assert_eq!(renderer.shaders_id.len(), 1);
    }

    #[test]
    #[serial]
    fn test_opengl_init_entity_already_initialized() {
        let config = GlfwConfig::create().unwrap();
        let window = config
            .create_window("test_opengl_init_entity", Resolution::default())
            .unwrap();
        window.set_current().unwrap();

        let color = Color::default();
        let vertices = Triangle::new([0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
        let shader = Rc::new(ShaderSource::new(
            UNIFORM_TRIANGLE_VERT,
            UNIFORM_TRIANGLE_FRAG,
        ));
        let entity = View::new(1, Some(&color), Some(&vertices), Some(shader.clone()), None);

        let second_entity = View::new(1, None, None, None, None);

        let mut renderer = OpenGL::default();

        assert!(renderer.init_entity(entity).is_ok());
        assert!(renderer.init_entity(second_entity).is_ok());

        assert_eq!(renderer.compiled_shaders.len(), 1);
        assert_eq!(renderer.buffers.len(), 1);
        assert_eq!(renderer.shaders_id.len(), 1);
    }

    #[test]
    #[serial]
    fn test_opengl_init_entity_compiled_shaders_reuse_shader() {
        let config = GlfwConfig::create().unwrap();
        let window = config
            .create_window("test_opengl_init_entity", Resolution::default())
            .unwrap();
        window.set_current().unwrap();

        let color = Color::default();
        let vertices = Triangle::new([0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
        let shader = Rc::new(ShaderSource::new(
            UNIFORM_TRIANGLE_VERT,
            UNIFORM_TRIANGLE_FRAG,
        ));
        let entity = View::new(1, Some(&color), Some(&vertices), Some(shader.clone()), None);

        let mut renderer = OpenGL::default();
        assert!(renderer.init_entity(entity).is_ok());

        let second_entity = View::new(2, Some(&color), Some(&vertices), Some(shader), None);

        assert!(renderer.init_entity(second_entity).is_ok());
        assert_eq!(renderer.compiled_shaders.len(), 1);
        assert_eq!(renderer.buffers.len(), 2);
        assert_eq!(renderer.shaders_id.len(), 2);

        //check if the same shader is used
        assert_eq!(
            renderer.shaders_id.get(&1).unwrap(),
            renderer.shaders_id.get(&2).unwrap()
        );
    }
}
