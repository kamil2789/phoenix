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
    events::{
        action::Action,
        condition::Condition,
        keys_binding::{KEY_A, KEY_D, KEY_S, KEY_W},
        user_input::{KeyboardInput, MouseInput},
        Event, EventLifetime,
    },
    renderer::opengl::OpenGL,
    systems::{camera, scaler::Scaler, scene::Scene},
    window::{GlfwConfig, Resolution},
};
use std::rc::Rc;

pub fn start_demo() {
    let config = GlfwConfig::create().unwrap();
    let window = Rc::new(
        config
            .create_window(
                "OpenGL",
                Resolution {
                    width: 1600,
                    height: 900,
                },
            )
            .unwrap(),
    );

    window.set_current();
    window.set_capture_mouse(true);

    let render = Box::new(OpenGL::new(window.as_ref()).unwrap());
    let scaler = Scaler::new(window.get_resolution());

    let mut scene = Scene::new(window, render);
    scene.event_manager.bind_key(
        KeyboardInput::new_key(KEY_A.into()),
        Action::CameraUpdateLeft,
    );
    scene.event_manager.bind_key(
        KeyboardInput::new_key(KEY_D.into()),
        Action::CameraUpdateRight,
    );

    scene.event_manager.bind_key(
        KeyboardInput::new_key(KEY_W.into()),
        Action::CameraUpdateForward,
    );

    scene.event_manager.bind_key(
        KeyboardInput::new_key(KEY_S.into()),
        Action::CameraUpdateBackward,
    );

    scene
        .event_manager
        .bind_mouse(MouseInput::Scroll, Action::CameraFov(0.0));

    scene
        .event_manager
        .bind_mouse(MouseInput::CursorPos, Action::CameraOrientation(0.0, 0.0));

    scene.event_manager.add_event(Event::new(
        EventLifetime::PerFrame,
        Condition::None,
        Action::ChangeBackgroundColor(RGBA::from_hex(0x00_FF_00_FF)),
    ));

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
    let light = Light::default();
    let lamp = Entity::new(vec![
        Component::Geometry(Box::new(cube_two)),
        Component::Color(Color::from_hex(0xff_ff_ff_00)),
        Component::Light(light),
    ]);

    let radius = scaler.radius(0.25);
    let sphere = Sphere::new(&Point::new_normalized(0.0, 0.0, 0.0), &radius, 60);

    let mut sphere_ent = Entity::default();
    sphere_ent.add_component(Component::Geometry(Box::new(sphere)));
    sphere_ent.add_component(Component::Color(Color::from_hex(0xFF_00_00_FF)));

    scene.add_entity(sphere_ent);
    scene.add_entity(entity);
    scene.add_entity(lamp);
    scene.set_background_color(RGBA::from_hex(0x00_00_00_FF));
    scene.register_camera(&camera::Config::default());
    scene.set_current_window().unwrap();
    scene.enable_3d();

    scene.start().unwrap();
}
