use std::collections::HashMap;
use std::rc::Rc;

use cgmath::Matrix4;
use geometry_rendering::{set_uniform_bool, set_uniform_color, set_uniform_matrix4f};

use super::{Error, Render, ID};
use crate::components::color::{Color, RGBA};
use crate::components::shaders::ShaderSource;
use crate::components::texture::Texture;
use crate::components::transformer::Transformer;
use crate::components::{Shape, ShapeType};
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
    shapes_type: HashMap<EntityID, ShapeType>,
    textures: HashMap<EntityID, TextureID>,
}

#[derive(Clone, Default)]
struct Buffers {
    pub vertex_array_object: u32,
    pub vertex_buffer_object: u32,
    pub indices: u16
}

impl Buffers {
    fn new(vertex_array_object: u32, vertex_buffer_object: u32, indices: u16) -> Buffers {
        Buffers {
            vertex_array_object,
            vertex_buffer_object,
            indices,
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
            if gl::IsEnabled(gl::DEPTH_TEST) == gl::TRUE {
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            } else {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
        }
    }

    /// Return the ID of the initialized entity
    fn init_entity(&mut self, entity: View) -> Result<ID> {
        if self.buffers.contains_key(&entity.entity_id) {
            return Ok(entity.entity_id); //already initialized
        }

        if let Some(value) = entity.shader_src.as_ref() {
            let id = self.handle_shader(value.clone())?;
            self.shaders_id.insert(entity.entity_id, id);
        }

        if let Some(shape) = entity.shape {
            let buffers = OpenGL::handle_vertices(shape, &entity)?;
            self.buffers.insert(entity.entity_id, buffers);
            if let Some(shader_id) = self.shaders_id.get(&entity.entity_id) {
                OpenGL::set_uniform_shader_variables(&entity, *shader_id)?;
            }

            self.shapes_type.insert(entity.entity_id, shape.get_type());
        }

        if let Some(texture) = entity.texture {
            let texture_id = self.init_texture(texture)?;
            self.textures.insert(entity.entity_id, texture_id);
        }

        Ok(entity.entity_id)
    }

    fn draw_entity(&self, entity_id: ID) {
        unsafe {
            if let Some(shader) = self.shaders_id.get(&entity_id) {
                gl::UseProgram(*shader);
            }

            if let Some(texture) = self.textures.get(&entity_id) {
                gl::BindTexture(gl::TEXTURE_2D, *texture);
            }

            if let Some(buffer) = self.buffers.get(&entity_id) {
                gl::BindVertexArray(buffer.vertex_array_object);
                if let Some(shape_type) = self.shapes_type.get(&entity_id) {
                    match shape_type {
                        ShapeType::Triangle | ShapeType::Cube => {
                            gl::DrawArrays(gl::TRIANGLES, 0, buffer.indices.into());
                        }
                        ShapeType::Circle => 
                            gl::DrawArrays(gl::TRIANGLE_FAN, 1, i32::from(buffer.indices - 1)),
                        

                        ShapeType::Sphere => gl::DrawArrays(1, 0, buffer.indices.into()),
                    };
                }
            }
        }
    }

    fn init_texture(&mut self, texture: &Texture) -> Result<ID> {
        textures::init_texture(texture)
    }

    fn perform_transformations(
        &mut self,
        entity_id: ID,
        transformation: &Transformer,
    ) -> Result<()> {
        if let Some(shader) = self.shaders_id.get(&entity_id) {
            if let Some(translation) = transformation.get_translation_matrix() {
                set_uniform_matrix4f("translation", &translation, *shader)?;
            }
            if let Some(rotation) = transformation.get_rotation_matrix() {
                set_uniform_matrix4f("rotation", &rotation, *shader)?;
            }
            if let Some(scale) = transformation.get_scale_matrix() {
                set_uniform_matrix4f("scale", &scale, *shader)?;
            }
            Ok(())
        } else {
            Err(Error::TransformationError(format!(
                "No shader found for entity {entity_id}"
            )))
        }
    }

    fn perform_camera_transformation(
        &mut self,
        entity_id: ID,
        camera_matrix: &Matrix4<f32>,
    ) -> Result<()> {
        if let Some(shader) = self.shaders_id.get(&entity_id) {
            set_uniform_matrix4f("projection", camera_matrix, *shader)?;
        }
        Ok(())
    }

    //TODO ADD RESULT
    fn enable_3d(&self) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }
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
            ShapeType::Cube | ShapeType::Triangle | ShapeType::Circle | ShapeType::Sphere => {
                OpenGL::handle_shape(shape, entity)
            }
        }
    }

    fn handle_shape(shape: &dyn Shape, entity: &View) -> Result<Buffers> {
        if entity.texture.is_some() {
            if let Some(val) = entity.color {
                if let Some(tmp) = val.as_ref_vertices() {
                    return Ok(geometry_rendering::init_shape_with_color_and_texture(
                        shape.get_vertices(),
                        tmp,
                    ));
                }
            }
            Ok(geometry_rendering::init_shape_with_texture(
                shape.get_vertices(),
            ))
        } else if let Some(val_color) = entity.color {
            OpenGL::handle_colored_triangle(shape, val_color)
        } else {
            Ok(geometry_rendering::init_shape(shape.get_vertices()))
        }
    }

    fn handle_colored_triangle(shape: &dyn Shape, color: &Color) -> Result<Buffers> {
        if color.as_ref_uniform().is_some() {
            Ok(geometry_rendering::init_shape(shape.get_vertices()))
        } else if let Some(vert_color) = color.as_ref_vertices() {
            Ok(geometry_rendering::init_shape_with_color(
                shape.get_vertices(),
                vert_color,
            ))
        } else {
            Err(Error::RenderingError(
                "Not supported color type".to_string(),
            ))
        }
    }

    fn set_uniform_shader_variables(entity: &View, shader_id: u32) -> Result<()> {
        if let Some(color) = entity.color {
            if let Some(value) = color.as_ref_uniform() {
                set_uniform_color("color", value, shader_id)?;
                if entity.texture.is_some() {
                    set_uniform_bool("isUniformColor", shader_id)?;
                }
            }
        }

        Ok(())
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
    use crate::renderer::shaders::UNIFORM_TRIANGLE_FRAG;
    use crate::window::{GlfwConfig, Resolution};
    use crate::{
        components::{geometry::plane::Triangle, shaders::ShaderSource},
        entities::entity::View,
        renderer::{shaders::UNIFORM_TRIANGLE_VERT, Render},
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
