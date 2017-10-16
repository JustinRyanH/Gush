use std::fmt;
use std::result::Result;
use std::error::Error;
use std::string;
use std::io;

use winit;
use glium;

use image;
use gltf_importer;

/// Handle all possible errors
#[derive(Debug)]
pub enum AppError {
    /// Something Happen during Basic App operations
    InitError(String),
    /// Something happened with interacting with outside of process
    IoError(io::Error),
    /// Memory Error
    MemError(String, String),
    /// VFS Error
    VirtualFilesystemError(String),
    /// Gfx Error
    GfxError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AppError::InitError(ref s) => write!(f, "Initilization Error: {}", s),
            AppError::MemError(ref s, ref l) => write!(f, "Memory Error: {} at {}", s, l),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        match *self {
            AppError::InitError(_) => "Error with application initialization",
            AppError::IoError(_) => "Error with Input Output",
            AppError::MemError(_, _) => "Error with Memory Access",
            AppError::VirtualFilesystemError(_) => "Error with processing file data",
            AppError::GfxError(_) => "Error with gfx communication",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            AppError::InitError(ref _s) => None,
            AppError::IoError(ref e) => Some(e),
            AppError::MemError(_, _) => None,
            AppError::VirtualFilesystemError(_) => None,
            AppError::GfxError(_) => None,
        }
    }
}

/// Result Type for Application
pub type AppResult<T> = Result<T, AppError>;

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> AppError {
        AppError::IoError(e)
    }
}

impl From<string::FromUtf8Error> for AppError {
    fn from(e: string::FromUtf8Error) -> AppError {
        AppError::VirtualFilesystemError(format!(
            "Was able to parse string till {}",
            e.utf8_error().valid_up_to()
        ))
    }
}

impl From<image::ImageError> for AppError {
    fn from(e: image::ImageError) -> AppError {
        AppError::VirtualFilesystemError(format!("Error Reading Image: {:?}", e))
    }
}
impl From<gltf_importer::Error> for AppError {
    fn from(e: gltf_importer::Error) -> AppError {
        AppError::VirtualFilesystemError(format!("Error parsing gltf file: {:?}", e))
    }
}

impl From<winit::CreationError> for AppError {
    fn from(e: winit::CreationError) -> AppError {
        AppError::InitError(format!("Failed to Create Window: {:?}", e))
    }
}

impl From<glium::SwapBuffersError> for AppError {
    fn from(e: glium::SwapBuffersError) -> AppError {
        AppError::GfxError(format!("Failed to Create Window: {:?}", e))
    }
}

impl From<glium::backend::glutin::DisplayCreationError> for AppError {
    fn from(e: glium::backend::glutin::DisplayCreationError) -> AppError {
        AppError::InitError(format!("Failed to Create Window: {:?}", e))
    }
}
