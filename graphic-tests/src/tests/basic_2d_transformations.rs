use std::rc::Rc;

use cgmath::vec3;
use phoenix::{
    components::{
        color::{Color, RGBA},
        geometry::Triangle,
        transformer::Builder,
        Component,
    },
    entities::entity::Entity,
    renderer::Render,
    systems::scene::Scene,
    window::Window,
};

pub fn test_2d_triangle_translation(window: Rc<Window>, render: Box<dyn Render>) {
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
