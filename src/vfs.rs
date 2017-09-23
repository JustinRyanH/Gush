use std::fs;
use std::io::Read;
use std::path;
use std::env;

use error::{AppResult, AppError};

pub struct VFS {
    current_dir: path::PathBuf,
}

fn assert_directory_exists(current_dir: &path::PathBuf, directory: &str) -> AppResult<()> {
    let assets = match fs::metadata(current_dir.join(directory)) {
        Ok(a) => a,
        Err(_) => return Err(AppError::InitError(
            format!("{} directory is not located in: {:?}", directory, current_dir).into()
        )),
    };

    if !assets.is_dir() {
        return Err(AppError::InitError(
            format!("{} is not a directory", directory).into()
        ));
    }
    Ok(())
}

impl VFS {
    /// Generate new Virtual Filesystem and asserts
    /// that it has access to shader and assets directories
    pub fn new() -> AppResult<VFS> {
        let current_dir = env::current_dir()?;
        assert_directory_exists(&current_dir, "assets")?;
        assert_directory_exists(&current_dir, "shaders")?;
        Ok(VFS { current_dir })
    }

    /// Load Given file into collection of Vector of bytes
    fn load_file(&self, file: path::PathBuf) -> AppResult<Vec<u8>> {
        let mut buffer: Vec<u8> = Vec::new();
        let mut file = fs::File::open(file)?;
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    /// Load shader code as binary file
    pub fn load_shader_code(&self, file_name: &str) -> AppResult<String> {
        let fullpath = self.current_dir.join("shaders").join(file_name);
        let buffer = self.load_file(fullpath)?;
        match String::from_utf8(buffer) {
            Ok(k) => Ok(k),
            Err(e) => Err(e.into()),
        }
    }

    /// Load binary files from asset directory
    pub fn load_binary_asset(&self, file_name: &str) -> AppResult<Vec<u8>> {
        let fullpath = self.current_dir.join("assets").join(file_name);
        self.load_file(fullpath)
    }
}
