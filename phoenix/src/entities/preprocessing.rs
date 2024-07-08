use crate::{
    components::{shaders::ShaderSource, texture::Texture, transformer::Transformer, Component},
    renderer::shaders::{
        TEXTURE_TRIANGLE_FRAG, TEXTURE_TRIANGLE_VERT, UNIFORM_TRIANGLE_FRAG, UNIFORM_TRIANGLE_VERT,
        VERTICES_COLORED_TEXTURE_TRIANGLE_FRAG, VERTICES_COLORED_TEXTURE_TRIANGLE_VERT,
        VERTICES_COLORED_TRIANGLE_FRAG, VERTICES_COLORED_TRIANGLE_VERT,
    },
};

use super::entity::Entity;

#[must_use]
pub fn preprocessing(mut entity: Entity) -> Entity {
    //only triangle is supported
    if is_shader(&entity) {
        return entity;
    }

    if is_texture(&entity) && is_vertices_color(&entity) {
        insert_vertices_colored_texture_triangle(&mut entity);
    } else if is_texture(&entity) {
        insert_texture_triangle(&mut entity);
    } else if is_vertices_color(&entity) {
        insert_vertices_colored_triangle(&mut entity);
    } else {
        insert_uniform_triangle(&mut entity);
    }

    if !is_transformer(&entity) {
        entity.add_component(Component::Transformer(Transformer::default()));
    }

    entity
}

fn is_transformer(entity: &Entity) -> bool {
    entity.contains_component(&Component::Transformer(Transformer::default()))
}

fn is_shader(entity: &Entity) -> bool {
    entity.contains_component(&Component::ShaderProgram(ShaderSource::default()))
}

fn is_texture(entity: &Entity) -> bool {
    entity.contains_component(&Component::Texture(Texture::default()))
}

fn is_vertices_color(entity: &Entity) -> bool {
    if let Some(color) = entity.get_color() {
        color.is_vertices()
    } else {
        false
    }
}

fn insert_vertices_colored_texture_triangle(entity: &mut Entity) {
    entity.add_component(Component::ShaderProgram(
        create_default_vertices_colored_texture_shader(),
    ));
}

fn insert_texture_triangle(entity: &mut Entity) {
    entity.add_component(Component::ShaderProgram(create_default_texture_shader()));
}

fn insert_vertices_colored_triangle(entity: &mut Entity) {
    entity.add_component(Component::ShaderProgram(
        create_default_shader_vertex_color(),
    ));
}

fn insert_uniform_triangle(entity: &mut Entity) {
    entity.add_component(Component::ShaderProgram(
        create_default_shader_uniform_color(),
    ));
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

fn create_default_vertices_colored_texture_shader() -> ShaderSource {
    ShaderSource::new(
        VERTICES_COLORED_TEXTURE_TRIANGLE_VERT,
        VERTICES_COLORED_TEXTURE_TRIANGLE_FRAG,
    )
}

#[cfg(test)]
mod tests {
    use super::preprocessing;
    use crate::{
        components::{
            color::Color, plane_geometry::Triangle, shaders::ShaderSource, texture::Texture,
            Component,
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
        assert_eq!(4, result.len());
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
        assert_eq!(4, result.len());
    }

    #[test]
    fn test_add_default_shader_component() {
        let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

        let entity = Entity::new(vec![Component::Geometry(Box::new(Triangle::new(vertices)))]);

        assert_eq!(1, entity.len());
        let result = preprocessing(entity);
        assert!(result.contains_component(&Component::ShaderProgram(ShaderSource::default())));
        assert_eq!(3, result.len());
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
        assert_eq!(4, result.len());
    }
}
