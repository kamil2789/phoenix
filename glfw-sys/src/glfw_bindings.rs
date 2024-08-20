use std::ffi::{c_char, c_int, c_uchar, c_void};

pub static GLFW_CONTEXT_VERSION_MAJOR: c_int = 0x0002_2002;
pub static GLFW_CONTEXT_VERSION_MINOR: c_int = 0x0002_2003;
pub static GLFW_OPENGL_PROFILE: c_int = 0x0002_2008;
pub static GLFW_OPENGL_CORE_PROFILE: c_int = 0x0003_2001;
pub static GLFW_CURSOR_DISABLED: c_int = 0x0003_4003;
pub static GLFW_CURSOR: c_int = 0x0003_3001;
pub static GLFW_CURSOR_NORMAL: c_int = 0x0003_4001;

pub static GLFW_TRUE: c_int = 1;
pub static GLFW_FALSE: c_int = 0;

pub static GLFW_RELEASE: c_int = 0;
pub static GLFW_PRESS: c_int = 1;
pub static GLFW_REPEAT: c_int = 2;

pub type GLFWglproc = *const c_void;
pub type GLFWframebuffersizefun = extern "C" fn(*mut GLFWwindow, c_int, c_int);
pub type GLFWkeyfun = extern "C" fn(*mut GLFWwindow, c_int, c_int, c_int, c_int);
pub type GLFWcursorposfun = extern "C" fn(*mut GLFWwindow, f64, f64);
pub type GLFWscrollfun = extern "C" fn(*mut GLFWwindow, f64, f64);

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
    pub fn glfwGetCurrentContext() -> *mut GLFWwindow;
    pub fn glfwGetFramebufferSize(window: *mut GLFWwindow, width: *mut c_int, height: *mut c_int);
    pub fn glfwWindowShouldClose(window: *mut GLFWwindow) -> c_int;
    pub fn glfwSwapBuffers(window: *mut GLFWwindow);
    pub fn glfwPollEvents();
    pub fn glfwGetKey(window: *mut GLFWwindow, key: c_int) -> c_int;
    pub fn glfwSetWindowShouldClose(window: *mut GLFWwindow, value: c_int);
    pub fn glfwDestroyWindow(window: *mut GLFWwindow);
    pub fn glfwGetProcAddress(procname: *const c_uchar) -> GLFWglproc;
    pub fn glfwGetTime() -> f64;

    //callbacks
    pub fn glfwSetInputMode(window: *mut GLFWwindow, mode: c_int, value: c_int);
    pub fn glfwSetWindowUserPointer(window: *mut GLFWwindow, pointer: *const c_void);
    pub fn glfwGetWindowUserPointer(window: *mut GLFWwindow) -> *mut c_void;
    pub fn glfwSetKeyCallback(window: *mut GLFWwindow, callback: Option<GLFWkeyfun>);
    pub fn glfwSetFramebufferSizeCallback(
        window: *mut GLFWwindow,
        cbfun: Option<GLFWframebuffersizefun>,
    ) -> Option<GLFWframebuffersizefun>;
    pub fn glfwSetCursorPosCallback(
        window: *mut GLFWwindow,
        callback: Option<GLFWcursorposfun>,
    ) -> Option<GLFWcursorposfun>;
    pub fn glfwSetScrollCallback(
        window: *mut GLFWwindow,
        callback: Option<GLFWscrollfun>,
    ) -> Option<GLFWscrollfun>;
}

pub enum GLFWmonitor {}

pub enum GLFWwindow {}
