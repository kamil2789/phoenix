pub mod shader_program;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("File could not be opened, path: {0}")]
    SourceFileError(String),
    #[error("Error when loading data from a file ")]
    ReadFileError(#[from] std::io::Error),
    #[error("Shader compilation error {0}")]
    CompilationError(String),
    #[error("Shader linking program error {0}")]
    LinkError(String),
}
