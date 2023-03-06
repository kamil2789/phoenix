use std::env;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

#[cfg(windows)]
static DELIMETER: char = '\\';

#[cfg(unix)]
static DELIMETER: char = '/';

/// # Panics
#[must_use]
pub fn get_current_dir_name() -> String {
    let full_path = env::current_dir().unwrap();
    let (_, dir) = full_path.to_str().unwrap().rsplit_once(DELIMETER).unwrap();
    String::from(dir)
}

/// # Errors
///
/// Will return `Err` if `filename` does not exist or the user does not have
/// permission to read it.
/// # Panics
///
/// Will panic if file has an invalid format
pub fn read_src_from_file(path: &Path) -> Result<String, String> {
    let mut result = String::new();

    if path.is_file() {
        let mut file = OpenOptions::new()
            .read(true)
            .open(path.to_str().unwrap_or(""))
            .unwrap();
        file.read_to_string(&mut result).unwrap();
        Ok(result)
    } else {
        Err(format!(
            "File could not be opened, path: {}",
            path.to_str().unwrap()
        ))
    }
}

fn get_path_to_shaders() -> String {
    String::from("phoenix/src/shaders/")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_read_src_from_file_no_file() {
        let result = read_src_from_file(Path::new("/nonExistedPath"));
        assert!(result.is_err());
    }

    #[test]
    fn test_read_src_from_file_exists() {
        let text = "Hello World file reader";
        let file_name = "file_reader_test.txt";
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_name)
            .unwrap();
        let write_result = writeln!(file, "{}", text);
        assert!(write_result.is_ok());

        let result = read_src_from_file(Path::new(file_name)).unwrap();
        assert_eq!(result.trim(), text);

        assert!(fs::remove_file(file_name).is_ok());
    }
}