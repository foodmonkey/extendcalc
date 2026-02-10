// this is the data error for all the RON modules

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataError {
    // Automatic conversion from std::io::Error
    #[error("File system error: {0}")]
    Io(#[from] std::io::Error),

    // Automatic conversion from ron::error::SpannedError
    #[error("RON syntax error: {0}")]
    Ron(#[from] ron::error::SpannedError),
}
