use std::mem;
use std::{collections::HashMap, rc::Rc};

use crate::components::shaders::ShaderBase;
use crate::{
    components::{color::RGBA, geometry::Shape, shaders::ShaderSource, Component},
    renderer::shaders::{UNIFORM_TRIANGLE_FRAG, UNIFORM_TRIANGLE_VERT},
};
pub type ID = u32;

///Only one component of specific type can be added to the entity.
/// For example, only one color, one geometry, and one shader program.
/// Second geometry, color, or shader program will overwrite the previous one.
#[derive(Default)]
pub struct Entity {
    components: Vec<Component>,
}

#[derive(Default)]
pub struct Manager {
    colors: HashMap<ID, RGBA>,
    shaders_source: HashMap<ID, Rc<ShaderSource>>,
    shapes: HashMap<ID, Box<dyn Shape>>,
    id_gc: IdGarbageCollector,
    shader_base: ShaderBase,
}

pub struct View<'a> {
    pub entity_id: ID,
    pub color: Option<&'a RGBA>,
    pub shape: Option<&'a dyn Shape>,
    pub shader_src: Option<Rc<ShaderSource>>,
}

#[derive(Default)]
struct IdGarbageCollector {
    num_pool: u32,
    renewable_ids: Vec<ID>,
}

impl Entity {
    #[must_use]
    pub fn new(components: Vec<Component>) -> Self {
        Self { components }
    }

    pub fn add_component(&mut self, component: Component) {
        self.components.push(component);
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    #[must_use]
    pub fn contains_component(&self, component: &Component) -> bool {
        self.components
            .iter()
            .any(|item| mem::discriminant(item) == mem::discriminant(component))
    }
}

impl Manager {
    pub fn add_entity(&mut self, mut entity: Entity) -> ID {
        if entity.is_empty() {
            return 0;
        }

        entity = Manager::preprocessing(entity);

        let id = self.id_gc.create_id();
        for component in entity.components {
            match component {
                Component::Color(color) => {
                    self.colors.insert(id, color);
                }
                Component::Geometry(shape) => {
                    self.shapes.insert(id, shape);
                }
                Component::ShaderProgram(shader_program) => {
                    let tmp = self.shader_base.register_from_source(&shader_program);
                    self.shaders_source.insert(id, tmp);
                }
            }
        }

        id
    }

    pub fn remove_entity(&mut self, id: ID) {
        self.colors.remove(&id);
        self.shapes.remove(&id);
        self.shaders_source.remove(&id);
        self.id_gc.remove_id(id);
    }

    #[must_use]
    pub fn get_keys(&self) -> Vec<ID> {
        self.shapes.keys().copied().collect()
    }

    #[must_use]
    pub fn as_ref_entity(&self, key: ID) -> View {
        let mut shape = None;
        if let Some(value) = self.shapes.get(&key) {
            shape = Some(value.as_ref());
        }

        let mut shader = None;
        if let Some(value) = self.shaders_source.get(&key) {
            shader = Some(value.clone());
        }

        View::new(key, self.colors.get(&key), shape, shader)
    }

    fn preprocessing(mut entity: Entity) -> Entity {
        if !entity.contains_component(&Component::ShaderProgram(ShaderSource::default()))
            && entity.contains_component(&Component::Color(RGBA::default()))
        {
            entity.add_component(Component::ShaderProgram(
                Manager::create_default_shader_uniform_color(),
            ));
        }

        entity
    }

    fn create_default_shader_uniform_color() -> ShaderSource {
        ShaderSource::new(UNIFORM_TRIANGLE_VERT, UNIFORM_TRIANGLE_FRAG)
    }
}

impl<'a> View<'a> {
    #[must_use]
    pub fn new(
        entity_id: ID,
        color: Option<&'a RGBA>,
        shape: Option<&'a dyn Shape>,
        shader_src: Option<Rc<ShaderSource>>,
    ) -> Self {
        Self {
            entity_id,
            color,
            shape,
            shader_src,
        }
    }
}

impl IdGarbageCollector {
    pub fn create_id(&mut self) -> ID {
        if let Some(id) = self.renewable_ids.pop() {
            id
        } else {
            self.num_pool += 1;
            self.num_pool
        }
    }

    pub fn remove_id(&mut self, id: ID) {
        self.renewable_ids.push(id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::geometry::Triangle;

    #[test]
    fn test_new_manager() {
        let entity_manager = Manager::default();

        assert_eq!(entity_manager.colors.len(), 0);
        assert_eq!(entity_manager.shapes.len(), 0);
        assert_eq!(entity_manager.shaders_source.len(), 0);
    }

    #[test]
    fn test_add_full_entity() {
        let mut entity_manager = Manager::default();
        let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

        let mut entity = Entity {
            components: vec![
                Component::Color(RGBA::new(255, 0, 0, 255_f32)),
                Component::ShaderProgram(ShaderSource::new("", "")),
            ],
        };
        entity.add_component(Component::Geometry(Box::new(Triangle::new(vertices))));

        let id = entity_manager.add_entity(entity);

        assert_eq!(id, 1);
        assert_eq!(entity_manager.colors.len(), 1);
        assert_eq!(entity_manager.shapes.len(), 1);
        assert_eq!(entity_manager.shaders_source.len(), 1);
    }

    #[test]
    fn test_add_entity_only_geometry() {
        let mut entity_manager = Manager::default();
        let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

        let entity = Entity {
            components: vec![Component::Geometry(Box::new(Triangle::new(vertices)))],
        };

        let id = entity_manager.add_entity(entity);

        assert_eq!(id, 1);
        assert_eq!(entity_manager.colors.len(), 0);
        assert_eq!(entity_manager.shapes.len(), 1);
        assert_eq!(entity_manager.shaders_source.len(), 0);
    }

    #[test]
    fn test_add_entity_geometry_with_color_no_custom_shader() {
        let mut entity_manager = Manager::default();
        let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

        let entity = Entity {
            components: vec![
                Component::Geometry(Box::new(Triangle::new(vertices))),
                Component::Color(RGBA::new(255, 255, 0, 255_f32)),
            ],
        };

        let id = entity_manager.add_entity(entity);

        assert_eq!(id, 1);
        assert_eq!(entity_manager.colors.len(), 1);
        assert_eq!(entity_manager.shapes.len(), 1);
        assert_eq!(entity_manager.shaders_source.len(), 1);
    }

    #[test]
    fn test_add_entity_empty() {
        let mut entity_manager = Manager::default();
        let entity = Entity { components: vec![] };

        let id = entity_manager.add_entity(entity);

        assert_eq!(id, 0);
        assert_eq!(entity_manager.colors.len(), 0);
        assert_eq!(entity_manager.shapes.len(), 0);
        assert_eq!(entity_manager.shaders_source.len(), 0);
    }

    #[test]
    fn test_remove_entity() {
        let mut entity_manager = Manager::default();
        let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

        let entity = Entity {
            components: vec![
                Component::Color(RGBA::new(255, 0, 0, 255_f32)),
                Component::Geometry(Box::new(Triangle::new(vertices))),
                Component::ShaderProgram(ShaderSource::new("", "")),
            ],
        };

        let id = entity_manager.add_entity(entity);

        entity_manager.remove_entity(id);

        assert_eq!(entity_manager.colors.len(), 0);
        assert_eq!(entity_manager.shapes.len(), 0);
        assert_eq!(entity_manager.shaders_source.len(), 0);
    }

    #[test]
    fn test_remove_entity_no_existing_key() {
        let mut entity_manager = Manager::default();

        entity_manager.remove_entity(100);

        assert_eq!(entity_manager.colors.len(), 0);
        assert_eq!(entity_manager.shapes.len(), 0);
        assert_eq!(entity_manager.shaders_source.len(), 0);
    }

    #[test]
    fn test_get_keys_entity() {
        let mut entity_manager = Manager::default();
        let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

        let entity = Entity {
            components: vec![
                Component::Color(RGBA::new(255, 0, 0, 255_f32)),
                Component::Geometry(Box::new(Triangle::new(vertices))),
                Component::ShaderProgram(ShaderSource::new("", "")),
            ],
        };

        let second_entity = Entity {
            components: vec![
                Component::Color(RGBA::new(255, 0, 0, 255_f32)),
                Component::Geometry(Box::new(Triangle::new(vertices))),
                Component::ShaderProgram(ShaderSource::new("", "")),
            ],
        };

        let id = entity_manager.add_entity(entity);
        let second_id = entity_manager.add_entity(second_entity);

        let keys = entity_manager.get_keys();

        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&id));
        assert!(keys.contains(&second_id));
        assert_eq!(id, 1);
        assert_eq!(second_id, 2);
        assert_eq!(entity_manager.colors.len(), 2);
        assert_eq!(entity_manager.shapes.len(), 2);
        assert_eq!(entity_manager.shaders_source.len(), 2);
    }

    #[test]
    fn test_id_garbage_collector() {
        let mut entity_manager = Manager::default();
        let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

        let entity = Entity {
            components: vec![
                Component::Color(RGBA::new(255, 0, 0, 255_f32)),
                Component::Geometry(Box::new(Triangle::new(vertices))),
                Component::ShaderProgram(ShaderSource::new("", "")),
            ],
        };

        let second_entity = Entity {
            components: vec![
                Component::Color(RGBA::new(255, 0, 0, 255_f32)),
                Component::Geometry(Box::new(Triangle::new(vertices))),
                Component::ShaderProgram(ShaderSource::new("", "")),
            ],
        };

        let id = entity_manager.add_entity(entity);
        assert_eq!(id, 1);
        assert_eq!(entity_manager.id_gc.num_pool, 1);
        entity_manager.remove_entity(id);
        assert_eq!(entity_manager.id_gc.renewable_ids.len(), 1);
        assert_eq!(entity_manager.id_gc.num_pool, 1);

        let second_id = entity_manager.add_entity(second_entity);
        assert_eq!(second_id, 1);
        assert_eq!(entity_manager.id_gc.renewable_ids.len(), 0);
        entity_manager.remove_entity(second_id);
        assert_eq!(entity_manager.id_gc.num_pool, 1);
    }

    #[test]
    fn test_view() {
        let mut entity_manager = Manager::default();
        let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
        let entity = Entity {
            components: vec![
                Component::Color(RGBA::new(255, 0, 0, 255_f32)),
                Component::Geometry(Box::new(Triangle::new(vertices))),
                Component::ShaderProgram(ShaderSource::new("aa", "bb")),
            ],
        };
        let id = entity_manager.add_entity(entity);
        let view = entity_manager.as_ref_entity(id);

        assert_eq!(view.entity_id, id);
        assert_eq!(view.color.unwrap(), &RGBA::new(255, 0, 0, 255_f32));
        assert_eq!(view.shape.unwrap().get_vertices(), &vertices);

        let shader = view.shader_src.unwrap();
        assert_eq!(shader.get_vertex_shader(), "aa");
        assert_eq!(shader.get_fragment_shader(), "bb");
    }
}
