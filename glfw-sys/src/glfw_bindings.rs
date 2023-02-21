use std::ffi::{c_char, c_int, c_uchar, c_void};

pub static GLFW_CONTEXT_VERSION_MAJOR: c_int = 0x00022002;
pub static GLFW_CONTEXT_VERSION_MINOR: c_int = 0x00022003;
pub static GLFW_OPENGL_PROFILE: c_int = 0x00022008;
pub static GLFW_OPENGL_CORE_PROFILE: c_int = 0x00032001;

pub static GLFW_TRUE: c_int = 1;
pub static GLFW_FALSE: c_int = 0;

pub type GLFWglproc = *const c_void;
pub type GLFWframebuffersizefun = extern "C" fn(*mut GLFWwindow, c_int, c_int);

extern "C" {
    pub fn glfwInit() -> c_int;
    pub fn glfwTerminate();
    pub fn glfwWindowHint(hint: c_int, value: c_int);
    pub fn glfwCreateWindow(
        width: c_int,
        height: c_int,
        title: *const c_char,
        monitor: *mut GLFWmonitor,
        share: *mut GLFWwindow,
    ) -> *mut GLFWwindow;
    pub fn glfwMakeContextCurrent(window: *mut GLFWwindow);
    pub fn glfwSetFramebufferSizeCallback(
        window: *mut GLFWwindow,
        cbfun: *mut GLFWframebuffersizefun,
    ) -> *mut GLFWframebuffersizefun;
    pub fn glfwWindowShouldClose(window: *mut GLFWwindow) -> c_int;
    pub fn glfwSwapBuffers(window: *mut GLFWwindow);
    pub fn glfwPollEvents();
    pub fn glfwGetKey(window: *mut GLFWwindow, key: c_int) -> c_int;
    pub fn glfwSetWindowShouldClose(window: *mut GLFWwindow, value: c_int);
    pub fn glfwDestroyWindow(window: *mut GLFWwindow);
    pub fn glfwGetProcAddress(procname: *const c_uchar) -> GLFWglproc;
}

pub enum GLFWmonitor {}

pub enum GLFWwindow {}
