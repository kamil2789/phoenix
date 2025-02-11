use cgmath::{vec3, Vector3};
use phoenix::{
    components::{
        color::{Color, RGBA},
        geometry::{
            solid::{Cube, Sphere},
            Point,
        },
        light::Light,
        material::Material,
        texture::{
            generate_default_vertices_for_cube, load, Config, Filtering, MinFiltering, Texture,
            Wrapping,
        },
        transformer::Builder,
        Component,
    },
    entities::entity::Entity,
    renderer::Render,
    systems::{camera, scaler::Scaler, scene::Scene},
    window::Window,
};
use std::{collections::HashMap, path::Path, rc::Rc, sync::LazyLock};

use crate::workspace::TEST_TEXTURE_DIR;

use super::TestFunction;

pub static TEST_LIST: LazyLock<HashMap<String, TestFunction>> = LazyLock::new(|| {
    let mut tests: HashMap<String, TestFunction> = HashMap::new();
    tests.insert(
        "test_3d_light_gold_cube_with_basic_light".to_string(),
        test_3d_light_gold_cube_with_basic_light,
    );
    tests.insert(
        "test_3d_light_red_sphere_with_white_light".to_string(),
        test_3d_light_red_sphere_with_white_light,
    );
    tests.insert(
        "test_3d_light_orange_cube_with_green_light_source_light_translation".to_string(),
        test_3d_light_orange_cube_with_green_light_source_light_translation,
    );
    tests.insert(
        "test_3d_light_container_diffuse_specular_maps".to_string(),
        test_3d_light_container_diffuse_specular_maps,
    );
    tests
});

pub static OPENGL_NOT_SUPPORTED: LazyLock<Vec<String>> = LazyLock::new(Vec::new);

pub static VULKAN_NOT_SUPPORTED: LazyLock<Vec<String>> = LazyLock::new(|| {
    vec![
        "test_3d_light_gold_cube_with_basic_light".into(),
        "test_3d_light_red_sphere_with_white_light".into(),
        "test_3d_light_orange_cube_with_green_light_source_light_translation".into(),
        "test_3d_light_container_diffuse_specular_maps".into(),
    ]
});

pub fn test_3d_light_gold_cube_with_basic_light(window: Rc<Window>, render: Box<dyn Render>) {
    let mut scene = Scene::new(window, render);
    let cube = Cube::new(1.0, [0.0, 0.0, 0.0]);
    let mut entity = Entity::default();
    entity.add_component(Component::Geometry(Box::new(cube)));
    entity.add_component(Component::Color(Color::from_hex(0xFF_D7_00_FF)));
    entity.add_component(Component::Material(Material::default()));
    entity.add_component(Component::Transformer(
        Builder::new()
            .with_translation(vec3(0.0, 0.0, -6.0))
            .with_rotation(vec3(45.0, 45.0, 0.0))
            .build(),
    ));
    let cube_two = Cube::new(0.15, [-0.7, 0.3, -3.0]);
    let light = Light::default();
    let lamp = Entity::new(vec![
        Component::Geometry(Box::new(cube_two)),
        Component::Light(light),
    ]);
    scene.add_entity(entity);
    scene.add_entity(lamp);
    scene.set_background_color(RGBA::from_hex(0x00_00_00_FF));
    scene.register_camera(&camera::Config::default());
    scene.set_current_window().unwrap();
    scene.enable_3d();
    scene.start_one_frame().unwrap();
}

pub fn test_3d_light_red_sphere_with_white_light(window: Rc<Window>, render: Box<dyn Render>) {
    let scaler = Scaler::new(window.get_resolution());
    let mut scene = Scene::new(window, render);

    let radius = scaler.radius(0.25);
    let sphere = Sphere::new(&Point::new_normalized(0.0, 0.0, 0.0), &radius, 48);

    let mut entity = Entity::default();
    entity.add_component(Component::Geometry(Box::new(sphere)));
    entity.add_component(Component::Color(Color::from_hex(0xFF_00_00_FF)));
    entity.add_component(Component::Material(Material::new_shininess(32.0)));
    entity.add_component(Component::Transformer(
        Builder::new()
            .with_translation(vec3(0.0, 0.0, -2.0))
            .build(),
    ));

    let cube_two = Cube::new(0.1, [0.7, 0.3, -1.5]);
    let light = Light::default();
    let lamp = Entity::new(vec![
        Component::Geometry(Box::new(cube_two)),
        Component::Light(light),
    ]);

    scene.add_entity(entity);
    scene.add_entity(lamp);
    scene.set_background_color(RGBA::from_hex(0x00_00_00_FF));
    scene.register_camera(&camera::Config::default());
    scene.set_current_window().unwrap();
    scene.enable_3d();
    scene.start_one_frame().unwrap();
}

pub fn test_3d_light_orange_cube_with_green_light_source_light_translation(
    window: Rc<Window>,
    render: Box<dyn Render>,
) {
    let mut scene = Scene::new(window, render);
    let cube = Cube::new(1.0, [0.0, 0.0, 0.0]);
    let mut entity = Entity::default();
    entity.add_component(Component::Geometry(Box::new(cube)));
    entity.add_component(Component::Color(Color::from_hex(0xFF_AA_00_FF)));
    entity.add_component(Component::Transformer(
        Builder::new()
            .with_translation(vec3(0.0, 0.0, -6.0))
            .with_rotation(vec3(45.0, 45.0, 0.0))
            .build(),
    ));
    let cube_two = Cube::new(0.15, [-0.7, 0.3, -3.0]);
    let light = Light {
        ambient: Vector3::new(0.0, 0.2, 0.0),
        diffuse: Vector3::new(0.0, 1.0, 0.0),
        specular: Vector3::new(0.0, 1.0, 0.0),
    };
    let lamp = Entity::new(vec![
        Component::Geometry(Box::new(cube_two)),
        Component::Light(light),
        Component::Transformer(Builder::new().with_translation(vec3(0.0, 0.0, 0.0)).build()),
    ]);
    scene.add_entity(entity);
    scene.add_entity(lamp);
    scene.set_background_color(RGBA::from_hex(0x00_00_00_FF));
    scene.register_camera(&camera::Config::default());
    scene.set_current_window().unwrap();
    scene.enable_3d();
    scene.start_one_frame().unwrap();
}

pub fn test_3d_light_container_diffuse_specular_maps(window: Rc<Window>, render: Box<dyn Render>) {
    let mut scene = Scene::new(window, render);
    let cube = Cube::new(1.0, [0.0, 0.0, 0.0]);
    let mut entity = Entity::default();
    entity.add_component(Component::Geometry(Box::new(cube)));
    entity.add_component(Component::Material(Material::default()));
    entity.add_component(Component::Transformer(
        Builder::new()
            .with_translation(vec3(-0.5, 0.5, -3.0))
            .with_rotation(vec3(45.0, 0.0, 0.0))
            .build(),
    ));

    let texture_config = Config {
        wrapping_horizontal: Wrapping::Repeat,
        wrapping_vertical: Wrapping::Repeat,
        min_filtering: MinFiltering::Filtering(Filtering::Linear),
        max_filtering: Filtering::Linear,
    };

    let path = TEST_TEXTURE_DIR.to_owned() + "wodden_container.png";
    let texture_data = load(Path::new(&path)).unwrap();
    let wodden_container = Texture::new_with_vertices(
        texture_data,
        texture_config.clone(),
        generate_default_vertices_for_cube(),
    );

    let second_path = TEST_TEXTURE_DIR.to_owned() + "steel_frame.png";
    let second_texture_data = load(Path::new(&second_path)).unwrap();
    let steel_frame = Texture::new_with_vertices(
        second_texture_data,
        texture_config,
        generate_default_vertices_for_cube(),
    );

    entity.add_component(Component::Texture(wodden_container));
    entity.add_component(Component::Texture(steel_frame));

    let cube_two = Cube::new(0.15, [-1.0, -8.0, 0.0]);
    let light = Light::default();
    let lamp = Entity::new(vec![
        Component::Geometry(Box::new(cube_two)),
        Component::Light(light),
    ]);

    scene.add_entity(entity);
    scene.add_entity(lamp);
    scene.set_background_color(RGBA::from_hex(0x00_00_00_FF));
    scene.register_camera(&camera::Config::default());
    scene.set_current_window().unwrap();
    scene.enable_3d();
    scene.start_one_frame().unwrap();
}
