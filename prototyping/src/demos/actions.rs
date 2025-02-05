use cgmath::vec3;
use phoenix::{
    components::{
        color::{Color, RGBA},
        geometry::{
            solid::{Cube, Sphere},
            Point,
        },
        texture::{self, load, Filtering, MinFiltering, Mipmaps, Texture, Wrapping},
        transformer::Builder,
        Component, FillMode,
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
use std::{path::Path, rc::Rc};

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

    scene.event_manager.add_high_priority_event(Event::new(
        EventLifetime::Once,
        Condition::OnAction(Action::CameraUpdateBackward),
        Action::ChangeBackgroundColor(RGBA::from_hex(0xFF_00_00_FF)),
    ));

    scene.event_manager.add_event(Event::new(
        EventLifetime::PerFrame,
        Condition::OnAction(Action::CameraUpdateRight),
        Action::ChangeBackgroundColor(RGBA::from_hex(0x00_FF_00_FF)),
    ));

    scene.event_manager.add_event(Event::new(
        EventLifetime::PerFrame,
        Condition::None,
        Action::PrintFPS(),
    ));

    let cube = Cube::new(0.5, [0.0, 0.0, 0.0]);

    let texture_config = texture::Config {
        wrapping_horizontal: Wrapping::Repeat,
        wrapping_vertical: Wrapping::Repeat,
        min_filtering: MinFiltering::Mipmap(Mipmaps::LinearMipmapLinear),
        max_filtering: Filtering::Nearest,
    };

    let path = "graphic-tests/assets/textures/brickwall.jpg";
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

    let radius = scaler.radius(0.2);
    let mut sphere = Sphere::new(&Point::new_normalized(0.6, 0.6, 0.0), &radius, 16);

    sphere.set_fill_mode(FillMode::Lines);
    let mut entity_sphere = Entity::default();
    entity_sphere.add_component(Component::Geometry(Box::new(sphere)));
    entity_sphere.add_component(Component::Color(Color::from_hex(0xFF_00_00_FF)));
    entity_sphere.add_component(Component::Transformer(
        Builder::new()
            .with_translation(vec3(0.0, 0.0, -5.0))
            .build(),
    ));

    scene.add_entity(entity_sphere);

    scene.set_background_color(RGBA::from_hex(0xD3_FF_CC_FF));
    scene.register_camera(&camera::Config::default());
    scene.enable_3d();

    scene.start().unwrap();
}
