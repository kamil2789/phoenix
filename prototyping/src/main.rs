use phoenix::window::{GlfwConfig, Resolution};

fn main() {
    println!("Hello, world!");
    let config = GlfwConfig::create().unwrap();
    let window = config.create_window("OpenGL", Resolution{width: 800, height: 600}).unwrap();
    window.set_current();
}
