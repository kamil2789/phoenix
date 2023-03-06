use phoenix::graphics_api::{create_graphic_api, GraphicApiType};
use phoenix::shaders::shader_manager::ShaderManager;
use phoenix::window::create_window_lib_config;
use phoenix::window::{Library, Resolution};
use phoenix::shaders::utils::read_src_from_file;
use phoenix::color::RGBA;
use phoenix::polygons::Point;
use phoenix::polygons::TriangleVertices;
use phoenix::polygons::Triangle;

use std::path::Path;

fn main() {
    let win_lib = create_window_lib_config(&Library::GLFW).unwrap();
    let window = win_lib
        .create_window(
            Resolution {
                width: 800,
                height: 600,
            },
            "Hello World",
        )
        .unwrap();

    let graphic_api = create_graphic_api(&GraphicApiType::OpenGL, &window).unwrap();

    let mut shader_manager = ShaderManager::new(graphic_api.clone());
    let fragment_shader = read_src_from_file(Path::new("./phoenix/src/shaders/fragment/basic.frag")).unwrap();
    let vertex_shader = read_src_from_file(Path::new("./phoenix/src/shaders/vertex/basic.vert")).unwrap();
    let shader_id = shader_manager.compile_shader(&vertex_shader, &fragment_shader).unwrap();

    let triangle = Triangle::new(TriangleVertices::new(&[Point{x: 0.5_f32, y: 0.5_f32, z: 0_f32}, Point{x: 0_f32, y: 0_f32, z: 0_f32}, Point{x: -0.5_f32, y: -0.5_f32, z: 0_f32}]), shader_id, RGBA::from_hex(0xff_00_00_ff));
    //let triangle_api = graphic_api.create_triangle(&triangle).unwrap();
    //triangle_api.init().unwrap();

    while window.is_running() {
        graphic_api.draw_background(&RGBA::from_hex(0xff_00_00_00));
        //triangle_api.draw();
        window.swap_buffers();
    }
}
