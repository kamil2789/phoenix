use phoenix::{
    components::{
        color::RGBA,
        geometry::Triangle,
        shaders::shader_program::{read_src_from_file, ShaderProgram},
        Component,
    },
    managers::{entity::Entity, scene::Scene},
    renderer::opengl::OpenGL,
    window::{GlfwConfig, Resolution},
};
use std::{path::Path, rc::Rc};

fn main() {
    println!("Hello, world!");
    let config = GlfwConfig::create().unwrap();
    let window = Rc::new(
        config
            .create_window(
                "OpenGL",
                Resolution {
                    width: 800,
                    height: 600,
                },
            )
            .unwrap(),
    );

    window.set_current().unwrap();

    let render = Box::<OpenGL>::default();
    let mut scene = Scene::new(window, render);

    //env preparation
    let src = read_src_from_file(Path::new(
        "./phoenix/src/components/shaders/vertex/basic.vert",
    ))
    .unwrap();
    let src2 = read_src_from_file(Path::new(
        "./phoenix/src/components/shaders/fragment/basic.frag",
    ))
    .unwrap();

    let shader_program = ShaderProgram::new(&src, &src2);

    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0, // left
        0.5, -0.5, 0.0, // right
        0.0, 0.5, 0.0, // top
    ];

    let triangle = Triangle::new(vertices);
    let mut entity = Entity::default();
    entity.add_component(Component::ShaderProgram(shader_program));
    entity.add_component(Component::Geometry(Box::new(triangle)));

    scene.add_entity(entity);

    scene.set_background_color(RGBA::from_hex(0x00_FF_00_FF));
    scene.start().unwrap();
}
