use super::Buffers;
use std::ptr;

//RESULT TYPE SHOULD BE RESULT<T, ERR> - CHECK FOR ERR CASE
#[must_use]
pub fn init_triangle(vertices: &[f32]) -> Buffers {
    let buffers = generate_buffers();
    bind_buffers(&buffers);
    send_data_to_cpu_buffer(vertices);
    set_vertex_attribute_pointer();
    unbind_buffers();
    buffers
}

fn generate_buffers() -> Buffers {
    let mut vertex_array_object = 0;
    let mut vertex_buffer_object = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vertex_array_object);
        gl::GenBuffers(1, &mut vertex_buffer_object);
    }
    Buffers::new(vertex_array_object, vertex_buffer_object)
}

fn bind_buffers(buffers: &Buffers) {
    unsafe {
        gl::BindVertexArray(buffers.vertex_array_object);
        gl::BindBuffer(gl::ARRAY_BUFFER, buffers.vertex_buffer_object);
    }
}

fn send_data_to_cpu_buffer(vertices: &[f32]) {
    let size = vertices.len() * std::mem::size_of_val(vertices);
    unsafe {
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size.try_into().unwrap(),
            vertices.as_ptr().cast::<std::ffi::c_void>(),
            gl::STATIC_DRAW,
        );
    }
}

fn set_vertex_attribute_pointer() {
    let stride = 3 * std::mem::size_of::<f32>();
    unsafe {
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride.try_into().unwrap(),
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
    }
}

fn unbind_buffers() {
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
}
