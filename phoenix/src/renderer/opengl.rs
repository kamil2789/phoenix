use std::collections::HashMap;
use std::ffi::{c_void, CString};
use std::rc::Rc;

use cgmath::Matrix4;
use common::{set_uniform_bool, set_uniform_color, set_uniform_matrix4f};
use glfw_sys::glfw_bindings;

use super::{Error, Render, ID};
use crate::components::color::{Color, RGBA};
use crate::components::shaders::ShaderSource;
use crate::components::texture::Texture;
use crate::components::transformer::Transformer;
use crate::components::{FillMode, Shape, ShapeType};
use crate::entities::entity::View;
use crate::renderer::Result;
use crate::window::Window;

mod common;
mod geometry_rendering;
mod shader_compiler;
mod textures;

pub type ShaderID = u32;
pub type TextureID = u32;
pub type EntityID = u32;

#[derive(Clone)]
pub struct OpenGL {
    shaders_id: HashMap<EntityID, ShaderID>,
    compiled_shaders: HashMap<Rc<ShaderSource>, ShaderID>,
    buffers: HashMap<EntityID, Buffers>,
    shapes_type: HashMap<EntityID, ShapeType>,
    textures: HashMap<EntityID, TextureID>,
    shape_fill_mode: HashMap<EntityID, u32>,
}

#[derive(Clone, Default)]
struct Buffers {
    pub vertex_array_object: u32,
    pub vertex_buffer_object: u32,
    pub indices: u16,
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
            let buffers = OpenGL::handle_shape(shape, entity.color, entity.texture)?;
            self.buffers.insert(entity.entity_id, buffers);
            if let Some(shader_id) = self.shaders_id.get(&entity.entity_id) {
                OpenGL::set_uniform_shader_variables(&entity, *shader_id)?;
            }

            self.shapes_type.insert(entity.entity_id, shape.get_type());
            self.shape_fill_mode.insert(
                entity.entity_id,
                OpenGL::match_fill_mode(shape.get_fill_mode()),
            );
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
                    let mode: u32 = *self
                        .shape_fill_mode
                        .get(&entity_id)
                        .unwrap_or(&gl::TRIANGLES);
                    match shape_type {
                        ShapeType::Triangle | ShapeType::Cube => {
                            gl::DrawArrays(mode, 0, buffer.indices.into());
                        }
                        ShapeType::Circle => gl::DrawArrays(mode, 1, i32::from(buffer.indices - 1)),

                        ShapeType::Sphere => gl::DrawArrays(mode, 0, buffer.indices.into()),
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
            let transformation_matrix = transformation.get_matrix();
            set_uniform_matrix4f("model", &transformation_matrix, *shader)
        } else {
            Err(Error::TransformationError(format!(
                "No shader found for entity {entity_id}"
            )))
        }
    }

    fn perform_camera_projection_transformation(
        &mut self,
        entity_id: ID,
        camera_matrix: &Matrix4<f32>,
    ) -> Result<()> {
        if let Some(shader) = self.shaders_id.get(&entity_id) {
            set_uniform_matrix4f("projection", camera_matrix, *shader)?;
        }
        Ok(())
    }

    fn perform_camera_position_transformation(
        &mut self,
        entity_id: ID,
        camera_matrix: &Matrix4<f32>,
    ) -> Result<()> {
        if let Some(shader) = self.shaders_id.get(&entity_id) {
            set_uniform_matrix4f("camera_pos", camera_matrix, *shader)?;
        }
        Ok(())
    }

    //TODO ADD RESULT
    fn enable_3d(&self) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }
    }

    fn get_last_error_code(&self) -> Option<u32> {
        let err_code = unsafe { gl::GetError() };
        if err_code == gl::NO_ERROR {
            None
        } else {
            Some(err_code)
        }
    }
}

impl OpenGL {
    /// # Errors
    ///
    /// Will return `Err` when the loading of OpenGL functions failed.
    pub fn new(window: &Window) -> Result<Self> {
        window.set_current();
        OpenGL::load_gl_functions()?;
        Ok(OpenGL {
            compiled_shaders: HashMap::new(),
            buffers: HashMap::new(),
            shaders_id: HashMap::new(),
            textures: HashMap::new(),
            shapes_type: HashMap::new(),
            shape_fill_mode: HashMap::new(),
        })
    }

    fn handle_shader(&mut self, shader: Rc<ShaderSource>) -> Result<ShaderID> {
        if let Some(val) = self.compiled_shaders.get(&shader) {
            return Ok(*val); //already compiled
        }

        let result = self.compile_shader_program(shader.clone())?;
        self.compiled_shaders.insert(shader, result);
        Ok(result)
    }

    fn match_fill_mode(mode: FillMode) -> u32 {
        match mode {
            FillMode::Lines => gl::LINES,
            FillMode::Solid => gl::TRIANGLES,
            FillMode::Fan => gl::TRIANGLE_FAN,
        }
    }

    fn handle_shape(
        shape: &dyn Shape,
        color: Option<&Color>,
        texture: Option<&Texture>,
    ) -> Result<Buffers> {
        geometry_rendering::init_shape(
            shape.get_vertices(),
            Color::unpack_vertices(color),
            Texture::unpack_vertices(texture),
        )
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

    fn load_gl_functions() -> Result<()> {
        gl::load_with(OpenGL::get_proc_address);
        if gl::DrawBuffer::is_loaded()
            && gl::GenTextures::is_loaded()
            && gl::GetVertexArrayIndexediv::is_loaded()
        {
            Ok(())
        } else {
            Err(Error::RenderingError(String::from(
                "GL functions were not loaded correctly",
            )))
        }
    }

    fn get_proc_address(func: &str) -> *const c_void {
        let c_style_name = CString::new(func.as_bytes()).unwrap();
        let ptr = c_style_name.as_ptr().cast::<u8>();
        unsafe { glfw_bindings::glfwGetProcAddress(ptr) }
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
        window.set_current();

        let color = Color::default();
        let vertices = Triangle::new([0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
        let shader = Rc::new(ShaderSource::new(
            UNIFORM_TRIANGLE_VERT,
            UNIFORM_TRIANGLE_FRAG,
        ));
        let entity = View::new(1, Some(&color), Some(&vertices), Some(shader), None);

        let mut renderer = OpenGL::new(&window).unwrap();
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
        window.set_current();

        let color = Color::default();
        let vertices = Triangle::new([0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
        let shader = Rc::new(ShaderSource::new(
            UNIFORM_TRIANGLE_VERT,
            UNIFORM_TRIANGLE_FRAG,
        ));
        let entity = View::new(1, Some(&color), Some(&vertices), Some(shader.clone()), None);

        let second_entity = View::new(1, None, None, None, None);

        let mut renderer = OpenGL::new(&window).unwrap();

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
        window.set_current();

        let color = Color::default();
        let vertices = Triangle::new([0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
        let shader = Rc::new(ShaderSource::new(
            UNIFORM_TRIANGLE_VERT,
            UNIFORM_TRIANGLE_FRAG,
        ));
        let entity = View::new(1, Some(&color), Some(&vertices), Some(shader.clone()), None);

        let mut renderer = OpenGL::new(&window).unwrap();
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
