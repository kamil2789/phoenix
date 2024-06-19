use crate::{
    components::color::RGBA,
    renderer::{Error, Result},
};

use super::Buffers;
use std::ffi::CString;

//RESULT TYPE SHOULD BE RESULT<T, ERR> - CHECK FOR ERR CASE
#[must_use]
pub fn init_triangle(vertices: &[f32]) -> Buffers {
    let buffers = generate_buffers();
    bind_buffers(&buffers);
    send_data_to_cpu_buffer(vertices);
    //layout 0, 3 vertices and no other attributes 0
    set_vertex_attribute_pointer(0, 3, 3, 0);
    unbind_buffers();
    buffers
}

pub fn init_triangle_with_color(position: &[f32], color: &[f32]) -> Buffers {
    let buffers = generate_buffers();
    bind_buffers(&buffers);
    let vertices = combine_position_with_color(position, color);
    send_data_to_cpu_buffer(&vertices);
    //position
    set_vertex_attribute_pointer(0, 3, 7, 0);
    //color
    set_vertex_attribute_pointer(1, 4, 7, 3);
    unbind_buffers();
    buffers
}

pub fn init_triangle_with_texture(position: &[f32]) -> Buffers {
    let buffers = generate_buffers();
    bind_buffers(&buffers);
    let vertices = combine_position_with_texture(position);
    send_data_to_cpu_buffer(&vertices);
    //position
    set_vertex_attribute_pointer(0, 3, 5, 0);
    //color
    set_vertex_attribute_pointer(1, 2, 5, 3);
    unbind_buffers();
    buffers
}

pub fn set_uniform_bool(variable_name: &str, shader_id: u32) -> Result<()> {
    let location = get_uniform_variable_location(shader_id, variable_name)?;
    unsafe {
        gl::UseProgram(shader_id);
        gl::Uniform1i(location, 1);
    };
    Ok(())
}

pub fn set_uniform_color(variable_name: &str, rgba: &RGBA, shader_id: u32) -> Result<()> {
    let color_location = get_uniform_variable_location(shader_id, variable_name)?;
    unsafe { gl::UseProgram(shader_id) };
    let color = rgba.get_as_normalized_f32();
    unsafe { gl::Uniform4f(color_location, color[0], color[1], color[2], color[3]) };
    Ok(())
}

fn get_uniform_variable_location(shader_id: u32, variable_name: &str) -> Result<i32> {
    if let Ok(name) = CString::new(variable_name) {
        let location = unsafe { gl::GetUniformLocation(shader_id, name.as_ptr()) };
        if location == -1 {
            Err(Error::RenderingError(
                "Uniform variable location not found".to_string(),
            ))
        } else {
            Ok(location)
        }
    } else {
        Err(Error::RenderingError(
            "Invalid variable name for uniform searching".to_string(),
        ))
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

fn combine_position_with_texture(position: &[f32]) -> Vec<f32> {
    let texture_vertices = [0.0, 0.0, 1.0, 0.0, 0.5, 1.0];
    let mut result = Vec::with_capacity(position.len() + texture_vertices.len());
    let pos_size = 3;
    let texture_size = 2;
    let iter = position
        .chunks(pos_size)
        .zip(texture_vertices.chunks(texture_size));

    for (pos, tex) in iter {
        result.extend_from_slice(pos);
        result.extend_from_slice(tex);
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

fn set_vertex_attribute_pointer(index: u32, size: i32, stride: usize, offset: usize) {
    let stride = stride * std::mem::size_of::<f32>();
    let offset = offset * std::mem::size_of::<f32>();
    unsafe {
        gl::VertexAttribPointer(
            index,
            size,
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
    use crate::{
        renderer::opengl::geometry_rendering::combine_position_with_texture,
        window::{GlfwConfig, Resolution},
    };
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

    #[test]
    fn test_combine_position_with_texture() {
        let position = vec![
            1_f32, 2_f32, 3_f32, 4_f32, 5_f32, 6_f32, 7_f32, 8_f32, 9_f32,
        ];

        let result = combine_position_with_texture(&position);
        assert_eq!(
            result,
            vec![
                1_f32, 2_f32, 3_f32, 0.0, 0.0, 4_f32, 5_f32, 6_f32, 1.0, 0.0, 7_f32, 8_f32, 9_f32,
                0.5, 1.0
            ]
        );
    }
}
