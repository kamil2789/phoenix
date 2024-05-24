use glfw_sys::glfw_bindings;
use phoenix::window::{GlfwConfig, Resolution};

fn main() {
    println!("Hello, world!");
    let config = GlfwConfig::create().unwrap();
    let window = config
        .create_window(
            "OpenGL",
            Resolution {
                width: 800,
                height: 600,
            },
        )
        .unwrap();

    window.set_current().unwrap();

    while window.is_running() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // gl::UseProgram(shader_program.get_id());
            // gl::BindVertexArray(triangle.get_vao());
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        unsafe {
            glfw_bindings::glfwPollEvents();
        }
    }
}
