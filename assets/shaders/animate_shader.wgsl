#import bevy_pbr::mesh_view_bindings::globals
#import bevy_pbr::forward_io::VertexOutput

struct Params {
    phase: f32,
    speed: f32,
    _pad: vec2<f32>, // padding to align to 16 bytes
}

@group(2) @binding(0) var<uniform> material: Params;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let t_1 = sin(material.phase) * 0.5 + 0.5;
    let t_2 = cos(material.phase);

    let distance_to_center = distance(in.uv, vec2<f32>(0.5)) * 1.4;

    // blending is done in a perceptual color space: https://bottosson.github.io/posts/oklab/
    let red = vec3<f32>(1.0, 0.0,0.0);
    let green = vec3<f32>(0.0,1.0,0.0);
    let blue = vec3<f32>(0.0, 0.0,1.0);
    let white = vec3<f32>(1.0, 1.0, 1.0);
    let mixed = mix(mix(red, blue, t_1), mix(green, white, t_2), distance_to_center);

    return vec4<f32>(mixed,1.0);
}
