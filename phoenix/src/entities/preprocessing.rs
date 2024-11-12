use crate::{
    components::{
        color::{Color, RGBA},
        light::Light,
        shaders::ShaderSource,
        Component,
    },
    renderer::shaders::{
        BASIC_SHAPES_FRAG, BASIC_SHAPES_VERT, LIGHT_SOURCE_FRAG, LIGHT_SOURCE_VERT,
    },
};

use super::entity::Entity;

#[must_use]
pub fn preprocessing(mut entity: Entity) -> Entity {
    if is_shader(&entity) {
        return entity;
    }

    if is_light_source(&entity) {
        insert_light_source_shader(&mut entity);
        if !entity.contains_component(&Component::Color(Color::default())) {
            entity.add_component(Component::Color(Color::from_rgba(RGBA::new_white())));
        }
        return entity;
    }

    insert_basic_shapers_shader(&mut entity);
    entity
}

fn is_shader(entity: &Entity) -> bool {
    entity.contains_component(&Component::ShaderProgram(ShaderSource::default()))
}

fn is_light_source(entity: &Entity) -> bool {
    entity.contains_component(&Component::Light(Light {}))
}

fn insert_light_source_shader(entity: &mut Entity) {
    entity.add_component(Component::ShaderProgram(
        create_default_light_source_shader(),
    ));
}

fn insert_basic_shapers_shader(entity: &mut Entity) {
    entity.add_component(Component::ShaderProgram(create_default_basic_shader()));
}

fn create_default_light_source_shader() -> ShaderSource {
    ShaderSource::new(LIGHT_SOURCE_VERT, LIGHT_SOURCE_FRAG)
}

fn create_default_basic_shader() -> ShaderSource {
    ShaderSource::new(BASIC_SHAPES_VERT, BASIC_SHAPES_FRAG)
}

#[cfg(test)]
mod tests {
    use super::preprocessing;
    use crate::{
        components::{
            color::Color, geometry::plane::Triangle, light::Light, shaders::ShaderSource,
            texture::Texture, Component,
        },
        entities::entity::Entity,
    };

    #[test]
    fn test_add_entity_geometry_with_color_rgba_no_custom_shader_add_shader() {
        let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

        let entity = Entity::new(vec![
            Component::Geometry(Box::new(Triangle::new(vertices))),
            Component::Color(Color::new(255, 255, 0, 255_f32)),
        ]);

        assert_eq!(2, entity.len());
        let result = preprocessing(entity);
        assert!(result.contains_component(&Component::ShaderProgram(ShaderSource::default())));
        assert_eq!(3, result.len());
    }

    #[test]
    fn test_add_entity_geometry_with_color_vertex_no_custom_shader_add_shader() {
        let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
        let color_vertices = vec![1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0];

        let entity = Entity::new(vec![
            Component::Geometry(Box::new(Triangle::new(vertices))),
            Component::Color(Color::from_vertices(color_vertices)),
        ]);

        assert_eq!(2, entity.len());
        let result = preprocessing(entity);
        assert!(result.contains_component(&Component::ShaderProgram(ShaderSource::default())));
        assert_eq!(3, result.len());
    }

    #[test]
    fn test_add_default_shader_component() {
        let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

        let entity = Entity::new(vec![Component::Geometry(Box::new(Triangle::new(vertices)))]);

        assert_eq!(1, entity.len());
        let result = preprocessing(entity);
        assert!(result.contains_component(&Component::ShaderProgram(ShaderSource::default())));
        assert_eq!(2, result.len());
    }

    #[test]
    fn test_add_default_shader_component_to_texture() {
        let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

        let entity = Entity::new(vec![
            Component::Geometry(Box::new(Triangle::new(vertices))),
            Component::Texture(Texture::default()),
        ]);

        assert_eq!(2, entity.len());
        let result = preprocessing(entity);
        assert!(result.contains_component(&Component::ShaderProgram(ShaderSource::default())));
        assert_eq!(3, result.len());
    }

    #[test]
    fn test_add_light_shader_component_to_texture() {
        let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

        let entity = Entity::new(vec![
            Component::Geometry(Box::new(Triangle::new(vertices))),
            Component::Color(Color::new(255, 255, 0, 255_f32)),
            Component::Light(Light {}),
        ]);

        assert_eq!(3, entity.len());
        let result = preprocessing(entity);
        assert!(result.contains_component(&Component::ShaderProgram(ShaderSource::default())));
        assert_eq!(4, result.len());
    }
}
