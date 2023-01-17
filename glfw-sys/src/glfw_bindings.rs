use std::ffi::c_int;

extern "C" {
    pub fn glfwInit() -> c_int;
    pub fn glfwTerminate();
}
