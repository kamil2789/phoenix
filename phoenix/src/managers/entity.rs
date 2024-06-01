use std::collections::HashMap;

use crate::components::{
    color::RGBA, geometry::Shape, shaders::shader_program::ShaderProgram, Component,
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
    shader_programs: HashMap<ID, ShaderProgram>,
    shapes: HashMap<ID, Box<dyn Shape>>,
    id_gc: IdGarbageCollector,
}

pub struct View<'a> {
    pub entity_id: ID,
    pub color: Option<&'a RGBA>,
    pub shape: Option<&'a dyn Shape>,
    pub shader_program: Option<&'a ShaderProgram>,
}

#[derive(Default)]
struct IdGarbageCollector {
    num_pool: u32,
    renewable_ids: Vec<ID>,
}

impl Entity {
    pub fn add_component(&mut self, component: Component) {
        self.components.push(component);
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }
}

impl Manager {
    pub fn add_entity(&mut self, entity: Entity) -> ID {
        if entity.is_empty() {
            return 0;
        }

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
                    self.shader_programs.insert(id, shader_program);
                }
            }
        }

        id
    }

    pub fn remove_entity(&mut self, id: ID) {
        self.colors.remove(&id);
        self.shapes.remove(&id);
        self.shader_programs.remove(&id);
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
        View::new(
            key,
            self.colors.get(&key),
            shape,
            self.shader_programs.get(&key),
        )
    }
}

impl<'a> View<'a> {
    fn new(
        entity_id: ID,
        color: Option<&'a RGBA>,
        shape: Option<&'a dyn Shape>,
        shader_program: Option<&'a ShaderProgram>,
    ) -> Self {
        Self {
            entity_id,
            color,
            shape,
            shader_program,
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
        assert_eq!(entity_manager.shader_programs.len(), 0);
    }

    #[test]
    fn test_add_full_entity() {
        let mut entity_manager = Manager::default();
        let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

        let mut entity = Entity {
            components: vec![
                Component::Color(RGBA::new(255, 0, 0, 255_f32)),
                Component::ShaderProgram(ShaderProgram::new("", "")),
            ],
        };
        entity.add_component(Component::Geometry(Box::new(Triangle::new(vertices))));

        let id = entity_manager.add_entity(entity);

        assert_eq!(id, 1);
        assert_eq!(entity_manager.colors.len(), 1);
        assert_eq!(entity_manager.shapes.len(), 1);
        assert_eq!(entity_manager.shader_programs.len(), 1);
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
        assert_eq!(entity_manager.shader_programs.len(), 0);
    }

    #[test]
    fn test_add_entity_empty() {
        let mut entity_manager = Manager::default();
        let entity = Entity { components: vec![] };

        let id = entity_manager.add_entity(entity);

        assert_eq!(id, 0);
        assert_eq!(entity_manager.colors.len(), 0);
        assert_eq!(entity_manager.shapes.len(), 0);
        assert_eq!(entity_manager.shader_programs.len(), 0);
    }

    #[test]
    fn test_remove_entity() {
        let mut entity_manager = Manager::default();
        let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

        let entity = Entity {
            components: vec![
                Component::Color(RGBA::new(255, 0, 0, 255_f32)),
                Component::Geometry(Box::new(Triangle::new(vertices))),
                Component::ShaderProgram(ShaderProgram::new("", "")),
            ],
        };

        let id = entity_manager.add_entity(entity);

        entity_manager.remove_entity(id);

        assert_eq!(entity_manager.colors.len(), 0);
        assert_eq!(entity_manager.shapes.len(), 0);
        assert_eq!(entity_manager.shader_programs.len(), 0);
    }

    #[test]
    fn test_remove_entity_no_existing_key() {
        let mut entity_manager = Manager::default();

        entity_manager.remove_entity(100);

        assert_eq!(entity_manager.colors.len(), 0);
        assert_eq!(entity_manager.shapes.len(), 0);
        assert_eq!(entity_manager.shader_programs.len(), 0);
    }

    #[test]
    fn test_get_keys_entity() {
        let mut entity_manager = Manager::default();
        let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

        let entity = Entity {
            components: vec![
                Component::Color(RGBA::new(255, 0, 0, 255_f32)),
                Component::Geometry(Box::new(Triangle::new(vertices))),
                Component::ShaderProgram(ShaderProgram::new("", "")),
            ],
        };

        let second_entity = Entity {
            components: vec![
                Component::Color(RGBA::new(255, 0, 0, 255_f32)),
                Component::Geometry(Box::new(Triangle::new(vertices))),
                Component::ShaderProgram(ShaderProgram::new("", "")),
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
        assert_eq!(entity_manager.shader_programs.len(), 2);
    }

    #[test]
    fn test_id_garbage_collector() {
        let mut entity_manager = Manager::default();
        let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

        let entity = Entity {
            components: vec![
                Component::Color(RGBA::new(255, 0, 0, 255_f32)),
                Component::Geometry(Box::new(Triangle::new(vertices))),
                Component::ShaderProgram(ShaderProgram::new("", "")),
            ],
        };

        let second_entity = Entity {
            components: vec![
                Component::Color(RGBA::new(255, 0, 0, 255_f32)),
                Component::Geometry(Box::new(Triangle::new(vertices))),
                Component::ShaderProgram(ShaderProgram::new("", "")),
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
                Component::ShaderProgram(ShaderProgram::new("aa", "bb")),
            ],
        };
        let id = entity_manager.add_entity(entity);
        let view = entity_manager.as_ref_entity(id);

        assert_eq!(view.entity_id, id);
        assert_eq!(view.color.unwrap(), &RGBA::new(255, 0, 0, 255_f32));
        assert_eq!(view.shape.unwrap().get_vertices(), &vertices);
        assert_eq!(view.shader_program.unwrap().get_vertex_shader(), "aa");
        assert_eq!(view.shader_program.unwrap().get_fragment_shader(), "bb");
    }
}
