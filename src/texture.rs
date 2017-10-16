use error::{AppResult};
use context::Context;
use image;


type Size = [u32; 2];

#[derive(Debug, Clone)]
pub struct Texture {
    pub size: Size,
}

impl Texture {
    pub fn load(ctx: &mut Context, path: &str) -> AppResult<Texture> {
        let asset = ctx.vfs.load_binary_asset(path)?;


        let img = image::load_from_memory(&asset)?.to_rgba();
        let (width, height) = img.dimensions();
        Texture::from_memory(ctx, width, height, &img)
    }

    pub fn from_memory(ctx: &mut Context, width: u32, height: u32, bytes: &[u8]) -> AppResult<Texture> {
        Ok(Texture {
            size: [width, height],
        })
    }
}
