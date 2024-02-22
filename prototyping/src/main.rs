use std::path::Path;

use glfw_sys::glfw_bindings;
use phoenix::{components::geometry::triangle::Triangle, render::init_triangle, components::shaders::{shader_source::{read_src_from_file, ShaderSrc}, shader_program::ShaderProgram}, window::{GlfwConfig, Resolution}};

fn main() {
    let config = GlfwConfig::create().unwrap();
    let window = config
        .create_window(
            "Learn OpenGL",
            Resolution {
                width: 800,
                height: 600,
            },
        )
        .unwrap();

    window.set_current().unwrap();

    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0, // left
         0.5, -0.5, 0.0, // right
         0.0,  0.5, 0.0  // top
    ];

    let mut triangle = Triangle::new(vertices);
    triangle = init_triangle(triangle);

    let src = read_src_from_file(Path::new("./phoenix/src/components/shaders/vertex/basic.vert")).unwrap();
    let src2 = read_src_from_file(Path::new("./phoenix/src/components/shaders/fragment/basic.frag")).unwrap();
    let shader_source = ShaderSrc::new(&src, &src2);
    let shader_program = ShaderProgram::new_compile(&shader_source).unwrap();
    while window.is_running() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(shader_program.get_id());
            gl::BindVertexArray(triangle.get_vao());
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
