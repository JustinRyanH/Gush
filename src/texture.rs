use gfx;
use gfx::traits::Factory;
use gfx_device_gl as gfx_gl;
use gfx::texture;

use graphics::types::ColorFormat;
use error::{AppResult};
use context::Context;
use image;


type Size = [u32; 2];

#[derive(Debug, Clone)]
pub struct Texture {
    pub size: Size,
    pub resource_view: gfx::handle::ShaderResourceView<gfx_gl::Resources, [f32; 4]>,
}

impl Texture {
    pub fn load(ctx: &mut Context, path: &str) -> AppResult<Texture> {
        let asset = ctx.vfs.load_binary_asset(path)?;


        let img = image::load_from_memory(&asset)?.to_rgba();
        let (width, height) = img.dimensions();
        Texture::from_memory(ctx, width, height, &img)
    }

    pub fn from_memory(ctx: &mut Context, width: u32, height: u32, bytes: &[u8]) -> AppResult<Texture> {
        let mut factory = ctx.gfx.get_factory_clone()?;

        let kind = gfx::texture::Kind::D2(
            width as gfx::texture::Size,
            height as gfx::texture::Size,
            texture::AaMode::Single,
        );

        let (_, resource_view) = factory.create_texture_immutable_u8::<ColorFormat>(
            kind,
            &[bytes],
        )?;
        Ok(Texture {
            size: [width, height],
            resource_view,
        })
    }
}
