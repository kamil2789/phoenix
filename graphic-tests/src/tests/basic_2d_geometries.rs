use std::rc::Rc;

use phoenix::{
    components::{color::RGBA, geometry::Triangle, Component},
    managers::{entity::Entity, scene::Scene},
    renderer::Render,
    window::Window,
};

pub fn test_2d_red_triangle_on_green_background(window: Rc<Window>, render: Box<dyn Render>) {
    let mut scene = Scene::new(window, render);

    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0, // left
        0.5, -0.5, 0.0, // right
        0.0, 0.5, 0.0, // top
    ];

    let triangle = Triangle::new(vertices);
    let mut entity = Entity::default();
    entity.add_component(Component::Geometry(Box::new(triangle)));
    entity.add_component(Component::Color(RGBA::from_hex(0xFF_00_00_FF)));

    scene.add_entity(entity);

    scene.set_background_color(RGBA::from_hex(0x00_FF_00_FF));
    scene.start_one_frame().unwrap();
}

pub fn test_2d_default_color_on_default_background(window: Rc<Window>, render: Box<dyn Render>) {
    let mut scene = Scene::new(window, render);

    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0, // left
        0.5, -0.5, 0.0, // right
        0.0, 0.5, 0.0, // top
    ];

    let triangle = Triangle::new(vertices);
    let mut entity = Entity::default();
    entity.add_component(Component::Geometry(Box::new(triangle)));

    scene.add_entity(entity);
    scene.start_one_frame().unwrap();
}

pub fn test_2d_two_triangles_green_blue(window: Rc<Window>, render: Box<dyn Render>) {
    let mut scene = Scene::new(window, render);

    let vertices: [f32; 9] = [
        -0.2, -0.2, 0.0, // left
        0.0, -0.0, 0.0, // right
        0.3, 0.0, 0.0, // top
    ];

    let second_vertices: [f32; 9] = [
        0.4, 0.4, 0.0, // left
        0.9, 0.9, 0.0, // right
        0.0, 0.9, 0.0, // top
    ];

    scene.set_background_color(RGBA::from_hex(0xFF_A5_90_FF));
    let triangle = Triangle::new(vertices);
    let green_triangle = Entity::new(vec![
        Component::Geometry(Box::new(triangle)),
        Component::Color(RGBA::from_hex(0x00_FF_00_FF)),
    ]);
    let blue_triangle = Entity::new(vec![
        Component::Geometry(Box::new(Triangle::new(second_vertices))),
        Component::Color(RGBA::from_hex(0x00_00_FF_FF)),
    ]);

    scene.add_entity(green_triangle);
    scene.add_entity(blue_triangle);
    scene.start_one_frame().unwrap();
}
