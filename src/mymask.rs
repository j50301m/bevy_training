use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;

/// 遮罩材質：main_texture 乘上 mask_texture 的 alpha
#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct MyMaskMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub main_texture: Handle<Image>,

    #[texture(2)]
    #[sampler(3)]
    pub mask_texture: Handle<Image>,
}

impl Material2d for MyMaskMaterial {
    /// 指定片段著色器，使用我們的 mask2D WGSL
    fn fragment_shader() -> ShaderRef {
        "shaders/mask2d.wgsl".into()
    }

    /// 頂點著色器沿用 Bevy 內建 2D shader 即可
    fn vertex_shader() -> ShaderRef {
        ShaderRef::Default
    }
}
