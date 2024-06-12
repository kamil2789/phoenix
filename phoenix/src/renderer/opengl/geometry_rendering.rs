use crate::components::color::{Color, RGBA};

use super::Buffers;
use std::{ffi::CString, ptr};

//RESULT TYPE SHOULD BE RESULT<T, ERR> - CHECK FOR ERR CASE
#[must_use]
pub fn init_triangle(vertices: &[f32]) -> Buffers {
    let buffers = generate_buffers();
    bind_buffers(&buffers);
    send_data_to_cpu_buffer(vertices);
    //layout 0, 3 vertices and no other attributes 0
    set_vertex_attribute_pointer(0, 3, 0);
    unbind_buffers();
    buffers
}

pub fn init_triangle_with_color(position: &[f32], color: &[f32]) -> Buffers {
    let buffers = generate_buffers();
    bind_buffers(&buffers);
    let vertices = combine_position_with_color(position, color);
    send_data_to_cpu_buffer(&vertices);
    //position
    set_vertex_attribute_pointer(0, 7, 0);
    //color
    set_vertex_attribute_pointer(1, 7, 3);
    unbind_buffers();
    buffers
}

//TODO ADD RESULT<>
pub fn set_uniform_color(variable_name: &str, color: &Color, shader_id: u32) {
    let uniform_color = CString::new(variable_name).unwrap();
    let color_location = unsafe { gl::GetUniformLocation(shader_id, uniform_color.as_ptr()) };
    unsafe { gl::UseProgram(shader_id) };
    if let Some(value) = color.as_ref_uniform() {
        let color = value.get_as_normalized_f32();
        unsafe { gl::Uniform4f(color_location, color[0], color[1], color[2], color[3]) };
    }
}

fn combine_position_with_color(position: &[f32], color: &[f32]) -> Vec<f32> {
    let mut result = Vec::with_capacity(position.len() + color.len());
    let pos_stride = 3;
    let color_stride = 4;
    let iter = position.chunks(pos_stride).zip(color.chunks(color_stride));

    for (pos, col) in iter {
        result.extend_from_slice(pos);
        result.extend_from_slice(col);
    }
    result
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

fn set_vertex_attribute_pointer(index: u32, stride: usize, offset: usize) {
    let stride = stride * std::mem::size_of::<f32>();
    let offset = offset * std::mem::size_of::<f32>();
    unsafe {
        gl::VertexAttribPointer(
            index,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride.try_into().unwrap_or(0),
            offset as *const std::ffi::c_void,
        );
        gl::EnableVertexAttribArray(index);
    }
}

fn unbind_buffers() {
    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
}

#[cfg(test)]
mod tests {
    use super::combine_position_with_color;
    use crate::window::{GlfwConfig, Resolution};
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_init_triangle() {
        let config = GlfwConfig::create().unwrap();
        let window = config
            .create_window("test_win_opengl", Resolution::default())
            .unwrap();
        window.set_current().unwrap();

        let vertices = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0];
        let buffers = super::init_triangle(&vertices);
        assert_ne!(buffers.vertex_array_object, 0);
        assert_ne!(buffers.vertex_buffer_object, 0);
    }

    #[test]
    fn test_combine_position_with_color() {
        let position = vec![
            1_f32, 2_f32, 3_f32, 4_f32, 5_f32, 6_f32, 7_f32, 8_f32, 9_f32,
        ];

        let color = vec![
            0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32,
            0.5_f32, 0.5_f32, 0.5_f32,
        ];

        let result = combine_position_with_color(&position, &color);
        assert_eq!(
            result,
            vec![
                1_f32, 2_f32, 3_f32, 0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 4_f32, 5_f32, 6_f32,
                0.5_f32, 0.5_f32, 0.5_f32, 0.5_f32, 7_f32, 8_f32, 9_f32, 0.5_f32, 0.5_f32, 0.5_f32,
                0.5_f32
            ]
        );
    }
}
