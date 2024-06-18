use std::{path::Path, rc::Rc};

use phoenix::{
    components::{
        color::{Color, RGBA},
        geometry::Triangle,
        texture::{Config, Filtering, MinFiltering, Texture, Wrapping},
        Component,
    },
    entities::entity::Entity,
    renderer::Render,
    systems::scene::Scene,
    window::Window,
};

use crate::workspace::TEST_TEXTURE_DIR;

pub fn test_2d_brick_wall_triangle(window: Rc<Window>, render: Box<dyn Render>) {
    let mut scene = Scene::new(window, render);

    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0, // left
        0.5, -0.5, 0.0, // right
        0.0, 0.5, 0.0, // top
    ];

    let triangle = Triangle::new(vertices);
    let mut entity = Entity::default();
    let texture_config = Config {
        wrapping_horizontal: Wrapping::Repeat,
        wrapping_vertical: Wrapping::Repeat,
        min_filtering: MinFiltering::Filtering(Filtering::Linear),
        max_filtering: Filtering::Linear,
    };

    let path = TEST_TEXTURE_DIR.to_owned() + "brickwall.jpg";
    let texture_data = Rc::new(image::open(Path::new(&path)).unwrap());
    let texture = Texture::new(texture_data, texture_config);
    entity.add_component(Component::Texture(texture));
    entity.add_component(Component::Geometry(Box::new(triangle)));

    scene.add_entity(entity);

    scene.set_background_color(RGBA::from_hex(0x00_B5_00_FF));
    if let Err(err) = scene.start_one_frame() {
        println!("{err}");
    }
}

pub fn test_2d_two_brick_wall_triangle(window: Rc<Window>, render: Box<dyn Render>) {
    let mut scene = Scene::new(window, render);

    let vertices: [f32; 9] = [
        -0.9, -0.9, 0.0, // left
        -0.5, -0.9, 0.0, // right
        -0.3, -0.1, 0.0, // top
    ];

    let second_vertices: [f32; 9] = [
        0.9, 0.9, 0.0, // left
        0.5, 0.9, 0.0, // right
        0.3, 0.1, 0.0, // top
    ];

    let triangle = Triangle::new(vertices);
    let mut entity = Entity::default();
    let texture_config = Config {
        wrapping_horizontal: Wrapping::Repeat,
        wrapping_vertical: Wrapping::Repeat,
        min_filtering: MinFiltering::Filtering(Filtering::Linear),
        max_filtering: Filtering::Linear,
    };

    let path = TEST_TEXTURE_DIR.to_owned() + "brickwall.jpg";
    let texture_data = Rc::new(image::open(Path::new(&path)).unwrap());
    let texture = Texture::new(texture_data, texture_config);
    entity.add_component(Component::Texture(texture.clone()));
    entity.add_component(Component::Geometry(Box::new(triangle)));

    scene.add_entity(entity);

    let second_entity = Entity::new(vec![
        Component::Geometry(Box::new(Triangle::new(second_vertices))),
        Component::Texture(texture),
    ]);
    scene.add_entity(second_entity);

    scene.set_background_color(RGBA::from_hex(0x00_B5_00_FF));
    if let Err(err) = scene.start_one_frame() {
        println!("{err}");
    }
}

pub fn test_2d_brick_wall_uniform_red_triangle(window: Rc<Window>, render: Box<dyn Render>) {
    let mut scene = Scene::new(window, render);

    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0, // left
        0.5, -0.5, 0.0, // right
        0.0, 0.5, 0.0, // top
    ];

    let triangle = Triangle::new(vertices);
    let mut entity = Entity::default();
    let texture_config = Config {
        wrapping_horizontal: Wrapping::Repeat,
        wrapping_vertical: Wrapping::Repeat,
        min_filtering: MinFiltering::Filtering(Filtering::Nearest),
        max_filtering: Filtering::Nearest,
    };

    let path = TEST_TEXTURE_DIR.to_owned() + "brickwall.jpg";
    let texture_data = Rc::new(image::open(Path::new(&path)).unwrap());
    let texture = Texture::new(texture_data, texture_config);
    entity.add_component(Component::Texture(texture));
    entity.add_component(Component::Geometry(Box::new(triangle)));
    entity.add_component(Component::Color(Color::from_hex(0xFF_00_00_FF)));

    scene.add_entity(entity);

    scene.set_background_color(RGBA::from_hex(0x00_B5_00_FF));
    if let Err(err) = scene.start_one_frame() {
        println!("{err}");
    }
}
