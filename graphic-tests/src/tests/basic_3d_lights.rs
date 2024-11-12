use cgmath::vec3;
use phoenix::{
    components::{
        color::{Color, RGBA},
        geometry::{
            solid::{Cube, Sphere},
            Point,
        },
        light::Light,
        transformer::Builder,
        Component,
    },
    entities::entity::Entity,
    renderer::Render,
    systems::{camera, scaler::Scaler, scene::Scene},
    window::Window,
};
use std::rc::Rc;
pub fn test_3d_gold_cube_with_basic_light(window: Rc<Window>, render: Box<dyn Render>) {
    let scaler = Scaler::new(window.get_resolution());
    let mut scene = Scene::new(window, render);
    let cube = Cube::new(0.5, [0.0, 0.0, 0.0]);
    let mut entity = Entity::default();
    entity.add_component(Component::Geometry(Box::new(cube)));
    entity.add_component(Component::Color(Color::from_hex(0xFF_D7_00_FF)));
    entity.add_component(Component::Transformer(
        Builder::new()
            .with_translation(vec3(0.0, 0.0, -3.0))
            .with_custom_axis_rotation_angle(vec3(0.5, 1.0, 0.0), 60.0)
            .build(),
    ));
    let radius = scaler.radius(0.1);
    let sphere = Sphere::new(&Point::new_normalized(0.3, -0.3, 0.0), &radius, 16);
    let light = Light {};
    let lamp = Entity::new(vec![
        Component::Geometry(Box::new(sphere)),
        Component::Color(Color::from_hex(0xaa_00_00_00)),
        Component::Transformer(
            Builder::new()
                .with_translation(vec3(1.0, 1.0, -4.0))
                .build(),
        ),
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
