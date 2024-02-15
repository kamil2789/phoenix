use std::ptr;

use crate::geometry::{basic::Buffers, triangle::Triangle};

//RESULT TYPE SHOULD BE RESULT<T, ERR> - CHECK FOR ERR CASE
pub fn init_triangle(mut triangle: Triangle) -> Triangle {
    let buffers = generate_buffers();
    bind_buffers(&buffers);
    send_data_to_cpu_buffer(triangle.get_vertices());
    set_vertex_attribute_pointer();
    unbind_bufferst();
    triangle.set_buffers(buffers);
    triangle
}

fn generate_buffers() -> Buffers {
    let mut vao = 0;
    let mut vbo = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
    }
    Buffers::new(vao, vbo)
}

fn bind_buffers(buffers: &Buffers) {
    unsafe {
        gl::BindVertexArray(buffers.get_vertex_array_object());
        gl::BindBuffer(gl::ARRAY_BUFFER, buffers.get_vertex_buffer_object());
    }
}

fn send_data_to_cpu_buffer(vertices: &[f32]) {
    let size = vertices.len() * std::mem::size_of::<f32>();
    unsafe {
        gl::BufferData(gl::ARRAY_BUFFER,
            size.try_into().unwrap(),
            vertices.as_ptr().cast::<std::ffi::c_void>(),
            gl::STATIC_DRAW);
    }
}

fn set_vertex_attribute_pointer() {
    let stride = 3 * std::mem::size_of::<f32>();
    unsafe {
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride.try_into().unwrap(), ptr::null());
        gl::EnableVertexAttribArray(0);
    }
}

fn unbind_bufferst() {
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
}
