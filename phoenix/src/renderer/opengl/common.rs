use cgmath::{Array, Matrix, Matrix4, Vector3};
use std::ffi::CString;

use crate::{
    components::color::RGBA,
    renderer::{Error, Result},
};

pub fn get_last_error_code(ignore_value_err: bool) -> Option<u32> {
    let error_code = unsafe { gl::GetError() };
    if error_code == gl::NO_ERROR || (error_code == gl::INVALID_VALUE && ignore_value_err) {
        None
    } else {
        Some(error_code)
    }
}

pub fn unset_uniform_bool(variable_name: &str, shader_id: u32) -> Result<()> {
    let location = get_uniform_variable_location(shader_id, variable_name)?;
    unsafe {
        gl::UseProgram(shader_id);
        gl::Uniform1i(location, 0);
    };
    Ok(())
}

pub fn set_uniform_bool(variable_name: &str, shader_id: u32) -> Result<()> {
    let location = get_uniform_variable_location(shader_id, variable_name)?;
    unsafe {
        gl::UseProgram(shader_id);
        gl::Uniform1i(location, 1);
    };
    Ok(())
}

pub fn set_uniform_vec3(variable_name: &str, vector: &Vector3<f32>, shader_id: u32) -> Result<()> {
    let location = get_uniform_variable_location(shader_id, variable_name)?;
    unsafe { gl::UseProgram(shader_id) };
    unsafe { gl::Uniform3fv(location, 1, vector.as_ptr())};
    Ok(())
}

pub fn set_uniform_matrix4f(
    variable_name: &str,
    matrix: &Matrix4<f32>,
    shader_id: u32,
) -> Result<()> {
    let location = get_uniform_variable_location(shader_id, variable_name)?;
    unsafe { gl::UseProgram(shader_id) };
    unsafe { gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr()) };
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
                "Uniform variable location not found: ".to_string() + variable_name,
            ))
        } else {
            Ok(location)
        }
    } else {
        Err(Error::RenderingError(
            "Invalid variable name for uniform searching: ".to_string() + variable_name,
        ))
    }
}
