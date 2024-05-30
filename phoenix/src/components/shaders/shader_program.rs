use super::Error;
use super::Result;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
pub struct ShaderProgram {
    vertex_shader: String,
    fragment_shader: String,
}

/// # Errors
///
/// Will return `Err` if file could not be opened or data in the file is corrupted.
pub fn read_src_from_file(path: &Path) -> Result<String> {
    let mut result = String::new();

    if let Ok(mut file) = OpenOptions::new()
        .read(true)
        .open(path.to_str().unwrap_or(""))
    {
        file.read_to_string(&mut result)?;
        Ok(result)
    } else {
        Err(Error::SourceFileError(
            path.to_str().unwrap_or("").to_string(),
        ))
    }
}

impl ShaderProgram {
    #[must_use]
    pub fn new(vertex: &str, fragment: &str) -> Self {
        ShaderProgram {
            vertex_shader: vertex.to_string(),
            fragment_shader: fragment.to_string(),
        }
    }

    /// # Errors
    ///
    /// Will return `Err` if `filename` does not exist or the user does not have
    /// permission to read it.
    pub fn new_from_file(vertex: &Path, fragment: &Path) -> Result<Self> {
        let vertex_shader = read_src_from_file(vertex)?;
        let fragment_shader = read_src_from_file(fragment)?;
        Ok(ShaderProgram {
            vertex_shader,
            fragment_shader,
        })
    }

    #[must_use]
    pub fn get_vertex_shader(&self) -> &str {
        &self.vertex_shader
    }

    #[must_use]
    pub fn get_fragment_shader(&self) -> &str {
        &self.fragment_shader
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    fn create_file(src: &str, file_name: &str) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_name)
            .unwrap();
        let write_result = writeln!(file, "{}", src);
        assert!(write_result.is_ok());
    }

    #[test]
    fn test_shader_src_new_from_non_existing_files_get_error() {
        let shader = ShaderProgram::new_from_file(
            Path::new("/nonExistedPath"),
            Path::new("/nonExistedPath"),
        );
        assert!(shader.is_err());

        if let Err(error) = shader {
            assert!(error
                .to_string()
                .contains("File could not be opened, path: /nonExistedPath"));
        }
    }

    #[test]
    fn test_shader_src_new_from_file() {
        let vertex_src = "vertex shader source code";
        let fragment_src = "fragment shader source code";
        let vertex_file = "vertex_shader.frag";
        let fragment_file = "fragment_shader.frag";

        create_file(vertex_src, vertex_file);
        create_file(fragment_src, fragment_file);

        let shader =
            ShaderProgram::new_from_file(Path::new(vertex_file), Path::new(fragment_file)).unwrap();

        assert_eq!(shader.get_vertex_shader().trim(), vertex_src);
        assert_eq!(shader.get_fragment_shader().trim(), fragment_src);

        assert!(fs::remove_file(vertex_file).is_ok());
        assert!(fs::remove_file(fragment_file).is_ok());
    }
}
