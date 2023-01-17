use glfw_sys::glfw_bindings::{glfwInit, glfwTerminate};

fn main() {
    println!("Hello, world!");
    unsafe {
        glfwInit();
        glfwTerminate();
    }
}
