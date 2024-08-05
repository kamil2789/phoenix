use std::{path::Path, rc::Rc};

use cgmath::vec3;
use phoenix::{
    components::{
        color::{Color, RGBA},
        geometry::solid::{Cube, Sphere},
        geometry::Point,
        texture::{self, load, Filtering, MinFiltering, Mipmaps, Texture, Wrapping},
        transformer::Builder,
        Component,
    },
    entities::entity::Entity,
    renderer::Render,
    systems::{camera, scene::Scene},
    window::Window,
};

use crate::workspace::TEST_TEXTURE_DIR;

pub fn test_3d_gold_cube_on_green_background(window: Rc<Window>, render: Box<dyn Render>) {
    let mut scene = Scene::new(window, render);

    let cube = Cube::new(0.5, [0.0, 0.0, 0.0]);

    let texture_config = texture::Config {
        wrapping_horizontal: Wrapping::Repeat,
        wrapping_vertical: Wrapping::Repeat,
        min_filtering: MinFiltering::Mimpmap(Mipmaps::LinearMipmapLinear),
        max_filtering: Filtering::Nearest,
    };

    let path = TEST_TEXTURE_DIR.to_owned() + "brickwall.jpg";
    let texture_data = load(Path::new(&path)).unwrap();
    let texture = Texture::new(texture_data, texture_config);

    let mut entity = Entity::default();
    entity.add_component(Component::Geometry(Box::new(cube)));
    entity.add_component(Component::Color(Color::from_hex(0xFF_D7_00_FF)));
    entity.add_component(Component::Texture(texture));
    entity.add_component(Component::Transformer(
        Builder::new()
            .with_translation(vec3(0.0, 0.0, -3.0))
            .with_custom_axis_rotation_angle(vec3(0.5, 1.0, 0.0), 60.0)
            .build(),
    ));

    scene.add_entity(entity);

    scene.set_background_color(RGBA::from_hex(0x56_7D_46_FF));
    scene.register_camera(&camera::Config::default());
    scene.set_current_window().unwrap();
    scene.enable_3d();
    scene.start_one_frame().unwrap();
}

pub fn test_3d_red_sphere_on_green_screen(window: Rc<Window>, render: Box<dyn Render>) {
    let mut scene = Scene::new(window, render);

    let sphere = Sphere::new(
        &Point::new_normalized(0.0, 0.0, 0.0),
        0.25,
        16,
    );

    let mut entity = Entity::default();
    entity.add_component(Component::Geometry(Box::new(sphere)));
    entity.add_component(Component::Color(Color::from_hex(0xFF_00_00_FF)));

    scene.add_entity(entity);

    scene.set_background_color(RGBA::from_hex(0x00_FF_00_FF));
    scene.set_current_window().unwrap();
    scene.enable_3d();
    scene.start_one_frame().unwrap();
}
