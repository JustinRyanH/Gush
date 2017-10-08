
use gfx::traits::FactoryExt;
use gltf::image::Data;
use gltf_utils::PrimitiveIterators;

use context::Context;
use error::{AppResult, AppError};
use graphics::types as graphic_types;
use graphics::pipeline::Vertex;
use texture::Texture;

pub struct SimpleMesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub texture: Option<Texture>,
}

impl SimpleMesh {
    pub fn from_gltf(ctx: &mut Context, gltf_path: &str) -> AppResult<SimpleMesh> {
        let mesh_index = 0;
        let (gltf, buffers) = ctx.vfs.load_gltf(gltf_path)?;
        if gltf.meshes().count() > 1 {
            return Err(AppError::VirtualFilesystemError(format!(
                "No Support for Multiple Meshes at this time: {}",
                gltf_path
            )));
        }
        // TODO: Handle Multiple Meshes
        if let Some(mesh) = gltf.meshes().nth(mesh_index) {
            // TODO: Handle Multiple Primitives
            if mesh.primitives().count() > 1 {
                return Err(AppError::VirtualFilesystemError(format!(
                    "Currently no Support for Multiple Primitives for mesh"
                )));
            }
            if let Some(primitive) = mesh.primitives().nth(0) {
                let mut vertices: Vec<Vertex> = match primitive.positions(&buffers) {
                    Some(position) => {
                        position
                            .map(|p| {
                                Vertex {
                                    pos: p.into(),
                                    ..Default::default()
                                }
                            })
                            .collect()
                    }
                    None => {
                        return Err(AppError::GfxError(
                            "Primitives must have an Position Attribute".into(),
                        ))
                    }
                };

                match primitive.tex_coords_f32(0, &buffers) {
                    Some(t) => {
                        for (i, uv) in t.enumerate() {
                            vertices[i].uv = uv.into();
                        }
                    }
                    None => (),
                }
                for v in &vertices {
                    println!("{:?}", v);
                }

                // TODO: Handle this more gracefully
                let indices: Vec<u32> = match primitive.indices_u32(&buffers) {
                    Some(i) => i.collect(),
                    None => {
                        return Err(AppError::VirtualFilesystemError(
                            "Mesh must have indices".into(),
                        ))
                    }
                };

                // TODO: Handle View Meshes
                let texture = match primitive
                    .material()
                    .pbr_metallic_roughness()
                    .base_color_texture() {
                    Some(base) => {
                        match base.texture().source().data() {
                            Data::View { .. } => None,
                            Data::Uri { uri, .. } => Some(Texture::load(ctx, uri)?),
                        }
                    }
                    None => None,
                };

                Ok(SimpleMesh {
                    vertices,
                    indices,
                    texture,
                })
            } else {
                Err(AppError::VirtualFilesystemError(format!(
                    "Primitives found for Mesh({}) in {}",
                    mesh_index,
                    gltf_path
                )))
            }
        } else {
            Err(AppError::VirtualFilesystemError(
                format!("No Mesh found in {}", gltf_path),
            ))
        }
    }

    pub fn generate_buffer(&self, ctx: &mut Context) -> AppResult<(graphic_types::GpuBuffer<Vertex>, graphic_types::Slice)> {
        let mut factory = ctx.gfx.get_factory_clone()?;
        Ok(factory.create_vertex_buffer_with_slice(self.vertices.as_slice(), self.indices.as_slice()))
    }
}
