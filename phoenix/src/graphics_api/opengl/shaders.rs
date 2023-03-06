use crate::graphics_api::{Result, GraphicApiError};

use std::ffi::CString;
use std::ptr;

pub fn delete(id: u32) {
    unsafe {
        gl::DeleteProgram(id);
    }
}

pub fn use_shader(id: u32) {
    unsafe {
        gl::UseProgram(id);
    }
}

pub fn compile(vertex_src: &str, fragment_src: &str) -> Result<u32> {
    unsafe {
        let shader_program_id = gl::CreateProgram();
        let vertex_shader_id = compile_shader(vertex_src, gl::VERTEX_SHADER)?;
        let fragment_shader_id = compile_shader(fragment_src, gl::FRAGMENT_SHADER)?;

        gl::AttachShader(shader_program_id, vertex_shader_id);
        gl::AttachShader(shader_program_id, fragment_shader_id);
        gl::LinkProgram(shader_program_id);
        check_link_status(shader_program_id)?;

        gl::DeleteShader(vertex_shader_id);
        gl::DeleteShader(fragment_shader_id);

        Ok(shader_program_id)
    }
}

unsafe fn compile_shader(src: &str, shader_type: u32) -> Result<u32> {
    let shader_id = gl::CreateShader(shader_type);
    let c_str_vert = CString::new(src.as_bytes()).unwrap();
    gl::ShaderSource(shader_id, 1, &c_str_vert.as_ptr(), ptr::null());
    gl::CompileShader(shader_id);

    check_compile_status(shader_id)?;
    Ok(shader_id)
}

unsafe fn check_compile_status(shader_id: u32) -> Result<()> {
    let mut status = i32::from(gl::TRUE);
    let info_length: usize = 512;
    let mut info_log: Vec<u8> = Vec::with_capacity(info_length - 1);
    gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut status);
    if status == i32::from(gl::TRUE) {
        Ok(())
    } else {
        gl::GetShaderInfoLog(
            shader_id,
            info_length.try_into().unwrap(),
            ptr::null_mut(),
            info_log.as_mut_ptr().cast::<i8>(),
        );

        Err(GraphicApiError::ShaderCompileError(
            std::str::from_utf8(&info_log).unwrap().to_string(),
        ))
    }
}

unsafe fn check_link_status(shader_program_id: u32) -> Result<()> {
    let mut status = i32::from(gl::FALSE);
    let info_length: usize = 512;
    let mut info_log: Vec<u8> = Vec::with_capacity(info_length - 1);
    gl::GetProgramiv(shader_program_id, gl::LINK_STATUS, &mut status);
    if status == i32::from(gl::TRUE) {
        Ok(())
    } else {
        gl::GetProgramInfoLog(
            shader_program_id,
            info_length.try_into().unwrap(),
            ptr::null_mut(),
            info_log.as_mut_ptr().cast::<i8>(),
        );

        Err(GraphicApiError::ShaderLinkError(
            std::str::from_utf8(&info_log).unwrap().to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::compile;
    use crate::graphics_api::create_opengl_api;
    use crate::window::{create_window_lib_config, Library};
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_compile_shaders() {
        let vertex_shader_src: &str = r#"
            #version 330 core
            layout (location = 0) in vec3 aPos;
            void main() {
                gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
            }
            "#;

        let fragment_shader_src: &str = r#"
            #version 330 core
            out vec4 FragColor;
            void main() {
                FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
            }
            "#;

        let config = create_window_lib_config(&Library::GLFW).unwrap();
        let window = config.create_default_window().unwrap();

        let _ = create_opengl_api(&window);
        let result = compile(vertex_shader_src, fragment_shader_src);
        assert!(result.is_ok());
    }
}
