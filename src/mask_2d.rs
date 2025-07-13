use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{AlphaMode2d, Material2d};

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct Mask2DMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub main_texture: Handle<Image>,

    #[texture(2)]
    pub mask_texture: Handle<Image>,
}

impl Material2d for Mask2DMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/mask2d.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        ShaderRef::Default
    }
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}
