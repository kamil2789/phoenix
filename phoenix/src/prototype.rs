use glfw_sys::glfw_bindings::{glfwInit, glfwTerminate};

pub unsafe fn prototyping() {
    glfwInit();
    glfwTerminate();
}
