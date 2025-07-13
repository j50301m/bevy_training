#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var main_texture : texture_2d<f32>;
@group(2) @binding(1) var main_sampler : sampler;
@group(2) @binding(2) var mask_texture : texture_2d<f32>;
@group(2) @binding(3) var mask_sampler : sampler;

@fragment
fn fragment(mesh: VertexOutput
) -> @location(0) vec4<f32> {
    let c = textureSample(main_texture, main_sampler, mesh.uv);
    let m = textureSample(mask_texture, mask_sampler, mesh.uv);
    return vec4<f32>(c.rgb, c.a * m.a);   // 只影響 alpha
}
