use phoenix::graphics_api::{create_graphic_api, GraphicApiType};
use phoenix::shaders::ShaderManager;
use phoenix::window::create_window_lib_config;
use phoenix::window::{Library, Resolution};

use phoenix::color::RGBA;

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

/* 
    let mut shader_manager = ShaderManager::new(graphic_api.clone());
    if let Err(_val) = shader_manager.compile_shader(" ", " ") {
        println!("DEBUG");
    }
*/

    while window.is_running() {
        graphic_api.draw_background(&RGBA::from_hex(0xff_00_00_00));
        window.swap_buffers();
    }
}
