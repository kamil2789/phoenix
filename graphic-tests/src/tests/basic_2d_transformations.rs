use std::{collections::HashMap, path::Path, rc::Rc, sync::LazyLock};

use cgmath::vec3;
use phoenix::{
    components::{
        color::{Color, RGBA},
        geometry::plane::Triangle,
        texture::{load, Config, Filtering, MinFiltering, Mipmaps, Texture, Wrapping},
        transformer::Builder,
        Component,
    },
    entities::entity::Entity,
    renderer::Render,
    systems::camera,
    systems::scene::Scene,
    window::Window,
};

use crate::workspace::TEST_TEXTURE_DIR;

use super::TestFunction;

pub static TEST_LIST: LazyLock<HashMap<String, TestFunction>> = LazyLock::new(|| {
    let mut tests: HashMap<String, TestFunction> = HashMap::new();
    tests.insert(
        "test_2d_transformation_triangle_translation".to_string(),
        test_2d_transformation_triangle_translation,
    );
    tests.insert(
        "test_2d_transformation_multiple_triangle_translation".to_string(),
        test_2d_transformation_multiple_triangle_translation,
    );
    tests.insert(
        "test_2d_transformation_triangle_rotation_and_scale".to_string(),
        test_2d_transformation_triangle_rotation_and_scale,
    );
    tests.insert(
        "test_2d_transformation_triangle_rotation_scale_perspective".to_string(),
        test_2d_transformation_triangle_rotation_scale_perspective,
    );
    tests
});

pub static OPENGL_NOT_SUPPORTED: LazyLock<Vec<String>> = LazyLock::new(Vec::new);

pub static VULKAN_NOT_SUPPORTED: LazyLock<Vec<String>> = LazyLock::new(|| {
    vec![
        "test_2d_transformation_triangle_translation".into(),
        "test_2d_transformation_multiple_triangle_translation".into(),
        "test_2d_transformation_triangle_rotation_and_scale".into(),
        "test_2d_transformation_triangle_rotation_scale_perspective".into(),
    ]
});

pub fn test_2d_transformation_triangle_translation(window: Rc<Window>, render: Box<dyn Render>) {
    let mut scene = Scene::new(window, render);

    let vertices: [f32; 9] = [
        -0.3, -0.3, 0.0, // left
        0.3, -0.3, 0.0, // right
        0.0, 0.3, 0.0, // top
    ];

    let colors = vec![
        1.0, 0.0, 0.0, 1.0, // left
        0.0, 1.0, 0.0, 1.0, // right
        0.0, 0.0, 1.0, 1.0, // top
    ];

    scene.set_background_color(RGBA::from_hex(0xFF_A5_90_FF));
    let position = Triangle::new(vertices);
    let triangle = Entity::new(vec![
        Component::Geometry(Box::new(position)),
        Component::Color(Color::from_vertices(colors)),
        Component::Transformer(Builder::new().with_translation(vec3(0.5, 0.5, 0.0)).build()),
    ]);

    scene.add_entity(triangle);
    scene.start_one_frame().unwrap();
}

pub fn test_2d_transformation_multiple_triangle_translation(
    window: Rc<Window>,
    render: Box<dyn Render>,
) {
    let mut scene = Scene::new(window, render);

    let vertices: [f32; 9] = [
        -0.3, -0.3, 0.0, // left
        0.3, -0.3, 0.0, // right
        0.0, 0.3, 0.0, // top
    ];

    let colors = vec![
        1.0, 0.0, 0.0, 1.0, // left
        0.0, 1.0, 0.0, 1.0, // right
        0.0, 0.0, 1.0, 1.0, // top
    ];

    scene.set_background_color(RGBA::from_hex(0xFF_A5_90_FF));
    let position = Triangle::new(vertices);
    let triangle = Entity::new(vec![
        Component::Geometry(Box::new(position)),
        Component::Color(Color::from_vertices(colors.clone())),
        Component::Transformer(Builder::new().with_translation(vec3(0.5, 0.5, 0.0)).build()),
    ]);

    let sec_vertices: [f32; 9] = [
        -0.1, -0.1, 0.0, // left
        0.1, -0.1, 0.0, // right
        0.0, 0.1, 0.0, // top
    ];

    let position = Triangle::new(sec_vertices);
    let sec_triangle = Entity::new(vec![
        Component::Geometry(Box::new(position)),
        Component::Color(Color::from_vertices(colors.clone())),
    ]);

    let third_vertices: [f32; 9] = [
        -0.1, -0.1, 0.0, // left
        0.1, -0.1, 0.0, // right
        0.0, 0.1, 0.0, // top
    ];

    let position = Triangle::new(third_vertices);
    let third_triangle = Entity::new(vec![
        Component::Geometry(Box::new(position)),
        Component::Color(Color::from_vertices(colors)),
        Component::Transformer(
            Builder::new()
                .with_translation(vec3(-0.5, -0.5, 0.0))
                .build(),
        ),
    ]);

    scene.add_entity(triangle);
    scene.add_entity(sec_triangle);
    scene.add_entity(third_triangle);
    scene.start_one_frame().unwrap();
}

pub fn test_2d_transformation_triangle_rotation_and_scale(
    window: Rc<Window>,
    render: Box<dyn Render>,
) {
    let mut scene = Scene::new(window, render);

    let vertices: [f32; 9] = [
        -0.3, -0.3, 0.0, // left
        0.3, -0.3, 0.0, // right
        0.0, 0.3, 0.0, // top
    ];

    let colors = vec![
        1.0, 0.0, 0.0, 1.0, // left
        0.0, 1.0, 0.0, 1.0, // right
        0.0, 0.0, 1.0, 1.0, // top
    ];

    scene.set_background_color(RGBA::from_hex(0xFF_A5_90_FF));
    let position = Triangle::new(vertices);
    let triangle = Entity::new(vec![
        Component::Geometry(Box::new(position)),
        Component::Color(Color::from_vertices(colors)),
        Component::Transformer(
            Builder::new()
                .with_rotation(vec3(80.0, 0.0, 0.0))
                .with_scale(vec3(2.0, 1.0, 1.0))
                .build(),
        ),
    ]);

    scene.add_entity(triangle);
    scene.start_one_frame().unwrap();
}

pub fn test_2d_transformation_triangle_rotation_scale_perspective(
    window: Rc<Window>,
    render: Box<dyn Render>,
) {
    let mut scene = Scene::new(window, render);

    let vertices: [f32; 9] = [
        -0.3, -0.3, 0.0, // left
        0.3, -0.3, 0.0, // right
        0.0, 0.3, 0.0, // top
    ];

    let colors = vec![
        1.0, 0.0, 0.0, 1.0, // left
        0.0, 1.0, 0.0, 1.0, // right
        0.0, 0.0, 1.0, 1.0, // top
    ];

    let triangle = Triangle::new(vertices);
    let texture_config = Config {
        wrapping_horizontal: Wrapping::Repeat,
        wrapping_vertical: Wrapping::Repeat,
        min_filtering: MinFiltering::Mipmap(Mipmaps::LinearMipmapLinear),
        max_filtering: Filtering::Linear,
    };

    let path = TEST_TEXTURE_DIR.to_owned() + "brickwall.jpg";
    let texture_data = load(Path::new(&path)).unwrap();
    let texture = Texture::new(texture_data, texture_config);

    scene.set_background_color(RGBA::from_hex(0xFF_A5_90_FF));

    let entity = Entity::new(vec![
        Component::Geometry(Box::new(triangle)),
        Component::Color(Color::from_vertices(colors)),
        Component::Texture(texture),
        Component::Transformer(
            Builder::new()
                .with_rotation(vec3(-55.0, 0.0, 0.0))
                .with_translation(vec3(0.0, 0.0, -3.0))
                .build(),
        ),
    ]);

    scene.add_entity(entity);
    scene.register_camera(&camera::Config {
        near_plane: 0.1,
        far_plane: 100.0,
        field_of_vision: 45.0,
        speed: 2.5,
        mouse_sensitivity: 0.1,
    });
    scene.start_one_frame().unwrap();
}
