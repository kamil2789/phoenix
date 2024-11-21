pub mod camera;
pub mod lighting;
pub mod performance;
pub mod scaler;
pub mod scene;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Lighting error {0}")]
    LightingError(String),
}
