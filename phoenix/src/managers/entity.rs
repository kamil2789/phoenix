use std::collections::HashMap;

use crate::components::{
    color::RGBA, geometry::Shape, shaders::shader_program::ShaderProgram, Component,
};
pub type ID = u32;

#[derive(Default)]
pub struct Entity {
    components: Vec<Component>,
}

#[derive(Default)]
pub struct Manager {
    colors: HashMap<ID, RGBA>,
    shader_programs: HashMap<ID, ShaderProgram>,
    shapes: HashMap<ID, Box<dyn Shape>>,
    entity_nums: u32,
}

pub struct View<'a> {
    pub entity_id: ID,
    pub color: Option<&'a RGBA>,
    pub shape: Option<&'a dyn Shape>,
    pub shader_program: Option<&'a ShaderProgram>,
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

impl Entity {
    pub fn add_component(&mut self, component: Component) {
        self.components.push(component);
    }

    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }
}

impl Manager {
    pub fn add_entity(&mut self, entity: Entity) {
        if entity.is_empty() {
            return;
        }

        self.entity_nums += 1;
        for component in entity.components {
            match component {
                Component::Color(color) => {
                    self.colors.insert(self.entity_nums, color);
                }
                Component::Geometry(shape) => {
                    self.shapes.insert(self.entity_nums, shape);
                }
                Component::ShaderProgram(shader_program) => {
                    self.shader_programs
                        .insert(self.entity_nums, shader_program);
                }
            }
        }
    }

    pub fn remove_entity(&mut self, id: ID) {
        self.colors.remove(&id);
        self.shapes.remove(&id);
        self.shader_programs.remove(&id);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::geometry::Triangle;

    #[test]
    fn test_new_scene_manager() {
        let scene_manager = Manager::default();

        assert_eq!(scene_manager.colors.len(), 0);
        assert_eq!(scene_manager.shapes.len(), 0);
        assert_eq!(scene_manager.shader_programs.len(), 0);
        assert_eq!(scene_manager.entity_nums, 0);
    }

    #[test]
    fn test_add_entity() {
        let mut scene_manager = Manager::default();
        let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

        let entity = Entity {
            components: vec![
                Component::Color(RGBA::new(255, 0, 0, 255_f32)),
                Component::Geometry(Box::new(Triangle::new(vertices))),
                Component::ShaderProgram(ShaderProgram::new("", "")),
            ],
        };

        scene_manager.add_entity(entity);

        assert_eq!(scene_manager.colors.len(), 1);
        assert_eq!(scene_manager.shapes.len(), 1);
        assert_eq!(scene_manager.shader_programs.len(), 1);
        assert_eq!(scene_manager.entity_nums, 1);
    }

    #[test]
    fn test_remove_entity() {
        let mut scene_manager = Manager::default();
        let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

        let entity = Entity {
            components: vec![
                Component::Color(RGBA::new(255, 0, 0, 255_f32)),
                Component::Geometry(Box::new(Triangle::new(vertices))),
                Component::ShaderProgram(ShaderProgram::new("", "")),
            ],
        };

        scene_manager.add_entity(entity);
        scene_manager.remove_entity(1);

        assert_eq!(scene_manager.colors.len(), 0);
        assert_eq!(scene_manager.shapes.len(), 0);
        assert_eq!(scene_manager.shader_programs.len(), 0);
    }
}
