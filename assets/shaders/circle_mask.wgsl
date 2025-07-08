

@fragment
fn fragment(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let center = vec2<f32>(0.5, 0.5);
    let dist = distance(uv, center);

    // // Normalize radius to UV coordinate space (0.0 to 1.0)
    if dist > material.radius / 200.0 { // normalize radius to 0~1
        discard;
    }

    return vec4<f32>(1.0, 1.0, 1.0, 1.0); // White color for the circle
}
