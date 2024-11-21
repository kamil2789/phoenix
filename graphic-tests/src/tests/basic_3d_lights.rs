use cgmath::vec3;
use phoenix::{
    components::{
        color::{Color, RGBA},
        geometry::solid::Cube,
        light::Light,
        transformer::Builder,
        Component,
    },
    entities::entity::Entity,
    renderer::Render,
    systems::{camera, scene::Scene},
    window::Window,
};
use std::rc::Rc;
pub fn test_3d_gold_cube_with_basic_light(window: Rc<Window>, render: Box<dyn Render>) {
    let mut scene = Scene::new(window, render);
    let cube = Cube::new(1.0, [0.0, 0.0, 0.0]);
    let mut entity = Entity::default();
    entity.add_component(Component::Geometry(Box::new(cube)));
    entity.add_component(Component::Color(Color::from_hex(0xFF_D7_00_FF)));
    entity.add_component(Component::Transformer(
        Builder::new()
            .with_translation(vec3(0.0, 0.0, -6.0))
            .with_rotation(vec3(45.0, 45.0, 0.0))
            .build(),
    ));
    let cube_two = Cube::new(0.15, [-0.7, 0.3, -3.0]);
    let light = Light {};
    let lamp = Entity::new(vec![
        Component::Geometry(Box::new(cube_two)),
        Component::Color(Color::from_hex(0xff_ff_ff_00)),
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
