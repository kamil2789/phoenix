pub mod shader_program;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("File could not be opened, path: {0}")]
    SourceFileError(String),
    #[error("Error when loading data from a file ")]
    ReadFileError(#[from] std::io::Error),
}
