// Compute shader for updating scratch card mask
@group(0) @binding(0) var mask_texture: texture_storage_2d<r8unorm, read_write>;

struct PaintData {
    position: vec2<f32>,
    radius: f32,
    paint_value: f32,
}

@group(0) @binding(1) var<uniform> paint_data: PaintData;

@compute @workgroup_size(8, 8, 1)
fn update_mask(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let tex_size = textureDimensions(mask_texture);
    let coords = global_id.xy;
    
    // Check if within texture bounds
    if (coords.x >= tex_size.x || coords.y >= tex_size.y) {
        return;
    }
    
    // Calculate distance from current pixel to brush center
    let pixel_pos = vec2<f32>(coords);
    let paint_pos = paint_data.position * vec2<f32>(tex_size);
    let distance = length(pixel_pos - paint_pos);
    
    // If within brush radius, update mask value
    if (distance <= paint_data.radius) {
        let current_value = textureLoad(mask_texture, coords).r;
        let new_value = max(current_value, paint_data.paint_value);
        textureStore(mask_texture, coords, vec4<f32>(new_value, 0.0, 0.0, 1.0));
    }
}