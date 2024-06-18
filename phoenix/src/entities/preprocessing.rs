use crate::{
    components::{shaders::ShaderSource, texture::Texture, Component},
    renderer::shaders::{
        TEXTURE_TRIANGLE_FRAG, TEXTURE_TRIANGLE_VERT, UNIFORM_TRIANGLE_FRAG, UNIFORM_TRIANGLE_VERT,
        VERTICES_COLORED_TRIANGLE_FRAG, VERTICES_COLORED_TRIANGLE_VERT,
    },
};

use super::entity::Entity;

#[must_use]
pub fn preprocessing(mut entity: Entity) -> Entity {
    if is_need_default_color_shader(&entity) {
        entity = add_default_shader_component_to_color(entity);
    } else if is_need_default_texture_shader(&entity) {
        entity = add_default_shader_component_to_texture(entity);
    }

    entity
}

fn add_default_shader_component_to_color(mut entity: Entity) -> Entity {
    if let Some(color) = entity.get_color() {
        if color.is_uniform() {
            entity.add_component(Component::ShaderProgram(
                create_default_shader_uniform_color(),
            ));
        } else if color.is_vertices() {
            entity.add_component(Component::ShaderProgram(
                create_default_shader_vertex_color(),
            ));
        }
    }

    entity
}

fn add_default_shader_component_to_texture(mut entity: Entity) -> Entity {
    entity.add_component(Component::ShaderProgram(create_default_texture_shader()));
    entity
}

fn is_need_default_texture_shader(entity: &Entity) -> bool {
    !entity.contains_component(&Component::ShaderProgram(ShaderSource::default()))
        && entity.contains_component(&Component::Texture(Texture::default()))
}

fn is_need_default_color_shader(entity: &Entity) -> bool {
    !entity.contains_component(&Component::ShaderProgram(ShaderSource::default()))
        && entity.get_color().is_some()
        && !entity.contains_component(&Component::Texture(Texture::default()))
}

fn create_default_shader_uniform_color() -> ShaderSource {
    ShaderSource::new(UNIFORM_TRIANGLE_VERT, UNIFORM_TRIANGLE_FRAG)
}

fn create_default_shader_vertex_color() -> ShaderSource {
    ShaderSource::new(
        VERTICES_COLORED_TRIANGLE_VERT,
        VERTICES_COLORED_TRIANGLE_FRAG,
    )
}

fn create_default_texture_shader() -> ShaderSource {
    ShaderSource::new(TEXTURE_TRIANGLE_VERT, TEXTURE_TRIANGLE_FRAG)
}

#[cfg(test)]
mod tests {
    use super::preprocessing;
    use crate::{
        components::{
            color::Color, geometry::Triangle, shaders::ShaderSource, texture::Texture, Component,
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
    fn test_add_default_shader_component_to_texture_no_texture() {
        let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

        let entity = Entity::new(vec![Component::Geometry(Box::new(Triangle::new(vertices)))]);

        assert_eq!(1, entity.len());
        let result = preprocessing(entity);
        assert!(!result.contains_component(&Component::ShaderProgram(ShaderSource::default())));
        assert_eq!(1, result.len());
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
}
