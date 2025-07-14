#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var reveal_texture : texture_2d<f32>;
@group(2) @binding(1) var reveal_sampler : sampler;
@group(2) @binding(2) var scratch_mask : texture_2d<f32>;
@group(2) @binding(3) var scratch_mask_sampler : sampler;
@group(2) @binding(4) var cover_texture : texture_2d<f32>;
@group(2) @binding(5) var cover_sampler : sampler;

const REPEAT_FACTOR : f32 = 3;


@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    // ---- 1. 取底圖顏色（獎品）
    let reveal_color  = textureSample(reveal_texture, reveal_sampler, mesh.uv);

    // ---- 2. 取遮罩灰階（假設 r 通道即 alpha）
    let mask_value    = textureSample(scratch_mask, scratch_mask_sampler, mesh.uv).r;

    // ---- 3. 以 fract() 做 repeat，取覆蓋層星星圖
    let pattern_uv    = fract(mesh.uv * REPEAT_FACTOR);
    let cover_color = textureSample(cover_texture, cover_sampler, pattern_uv);

    // ---- 4. 依遮罩灰階混合：
    //      mask_value = 0   -> 完全顯示 cover_color (星星圖案)
    //      mask_value = 1   -> 完全顯示 reveal_color (獎品)
    let out_rgb = mix(cover_color.rgb, reveal_color.rgb, mask_value);
    // 如果想要跟著 mask_value 改變透明度，就把下行改成 mask_value
    let out_a   = 1.0;

    return vec4<f32>(out_rgb, out_a);
}