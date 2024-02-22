use super::shader_source::ShaderSrc;
use super::Error;
use super::Result;
use std::ffi::CString;
use std::ptr;

pub struct ShaderProgram {
    id: u32,
}

impl ShaderProgram {
    /// # Errors
    ///
    /// Will return `Err` when the shader fails in the compilation or linking phase.
    /// The correct vertex and fragment shader should be given to this func.
    pub fn new_compile(shader_src: &ShaderSrc) -> Result<ShaderProgram> {
        let vertex_shader_id =
            ShaderProgram::compile_shader(shader_src.get_vertex_shader(), gl::VERTEX_SHADER)?;
        let fragment_shader_id =
            ShaderProgram::compile_shader(shader_src.get_fragment_shader(), gl::FRAGMENT_SHADER)?;
        let shader_program_id = ShaderProgram::create_program();
        ShaderProgram::link_program(shader_program_id, vertex_shader_id, fragment_shader_id)?;
        ShaderProgram::delete_shader(vertex_shader_id);
        ShaderProgram::delete_shader(fragment_shader_id);
        Ok(ShaderProgram {
            id: shader_program_id,
        })
    }
    
    pub fn get_id(&self) -> u32 {
        self.id
    }

    fn create_program() -> u32 {
        unsafe { gl::CreateProgram() }
    }

    fn compile_shader(shader_src: &str, shader_type: u32) -> Result<u32> {
        unsafe {
            let shader = gl::CreateShader(shader_type);
            let c_str_vert = CString::new(shader_src.as_bytes()).unwrap();
            gl::ShaderSource(shader, 1, &c_str_vert.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            ShaderProgram::check_compile_status(shader)
        }
    }

    fn link_program(shader_id: u32, vertex_id: u32, fragment_id: u32) -> Result<()> {
        unsafe {
            gl::AttachShader(shader_id, vertex_id);
            gl::AttachShader(shader_id, fragment_id);
            gl::LinkProgram(shader_id);
            ShaderProgram::check_link_status(shader_id)
        }
    }

    fn check_compile_status(shader_id: u32) -> Result<u32> {
        let mut status = i32::from(gl::TRUE);
        let info_length: usize = 512;
        let mut info_log: Vec<u8> = Vec::with_capacity(info_length - 1);
        unsafe { gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut status) };
        if status == i32::from(gl::TRUE) {
            Ok(shader_id)
        } else {
            unsafe {
                gl::GetShaderInfoLog(
                    shader_id,
                    info_length.try_into().unwrap(),
                    ptr::null_mut(),
                    info_log.as_mut_ptr().cast::<i8>(),
                );
            };
            Err(Error::CompilationError(String::from(
                std::str::from_utf8(&info_log).unwrap(),
            )))
        }
    }

    fn check_link_status(shader_id: u32) -> Result<()> {
        let mut status = i32::from(gl::TRUE);
        let info_length: usize = 512;
        let mut info_log: Vec<u8> = Vec::with_capacity(info_length - 1);
        unsafe { gl::GetProgramiv(shader_id, gl::LINK_STATUS, &mut status) };
        if status == i32::from(gl::TRUE) {
            Ok(())
        } else {
            unsafe {
                gl::GetProgramInfoLog(
                    shader_id,
                    info_length.try_into().unwrap(),
                    ptr::null_mut(),
                    info_log.as_mut_ptr().cast::<i8>(),
                );
            };
            Err(Error::LinkError(String::from(
                std::str::from_utf8(&info_log).unwrap(),
            )))
        }
    }

    fn delete_shader(shader_id: u32) {
        unsafe {
            gl::DeleteShader(shader_id);
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ShaderProgram;
    use crate::components::shaders::shader_program::ShaderSrc;
    use crate::window::GlfwConfig;
    use crate::window::Resolution;
    use crate::window::Window;
    use serial_test::serial;

    const VERTEX_SHADER_SRC: &str = r#"
        #version 330 core
        layout (location = 0) in vec3 aPos;
        void main() {
            gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
        }
    "#;

    const FRAGMENT_SHADER_SRC: &str = r#"
        #version 330 core
        out vec4 FragColor;
        void main() {
            FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
        }
    "#;

    fn set_glfw_window() -> (GlfwConfig, Window) {
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
        (config, window)
    }

    #[test]
    #[serial]
    fn test_compile_shader_no_error() {
        let (_config, _window) = set_glfw_window();

        let shader_src = ShaderSrc::new(VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC);
        let shader_program = ShaderProgram::new_compile(&shader_src);
        assert!(shader_program.is_ok());
    }

    #[test]
    #[serial]
    fn test_compile_shader_invalid_shaders_src_get_err() {
        let (_config, _window) = set_glfw_window();

        let mut shader_program = ShaderProgram::new_compile(&ShaderSrc::new(VERTEX_SHADER_SRC, "Invalidd data"));
        assert!(shader_program.is_err());

        shader_program = ShaderProgram::new_compile(&ShaderSrc::new("Invalidd data", FRAGMENT_SHADER_SRC));
        assert!(shader_program.is_err());

        shader_program = ShaderProgram::new_compile(&ShaderSrc::new("", ""));
        assert!(shader_program.is_err());
    }
}
