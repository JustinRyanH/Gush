use std::fmt;
use std::result::Result;
use std::error::Error;

/// Handle all possible errors
#[derive(Debug)]
pub enum AppError {
    /// Something Happen during Basic App operations
    InitError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AppError::InitError(ref s) => write!(f, "Initilization Error: {}", s),
        }
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        match *self {
            AppError::InitError(_) => "App Initilization Error",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            AppError::InitError(ref _s) => None,
        }
    }
}

/// Result Type for Application
pub type AppResult<T> = Result<T, AppError>;
