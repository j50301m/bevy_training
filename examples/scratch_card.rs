use bevy::{
    asset::RenderAssetUsages,
    image::ImageSampler,
    prelude::*,
    render::render_resource::{
        AsBindGroup, Extent3d, ShaderRef, TextureDimension, TextureFormat, TextureUsages,
    },
    sprite::{AlphaMode2d, Material2d, Material2dPlugin},
    window::PrimaryWindow,
};

const BRUSH_RADIUS: i32 = 20;

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct ScratchCardMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub reveal_texture: Handle<Image>,
    #[texture(2)]
    #[sampler(3)]
    pub scratch_mask: Handle<Image>,
    #[texture(4)]
    #[sampler(5)]
    pub cover_layer: Handle<Image>,
}

/// Scratch card resources structure storing texture handles and mask data
#[derive(Resource)]
pub struct ScratchResources {
    pub reveal_texture: Handle<Image>,
    pub cover_texture: Handle<Image>,
    pub mask_data: Vec<u8>,
    pub size: u32,
}

impl Material2d for ScratchCardMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/scratch_card.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        ShaderRef::Default
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(Material2dPlugin::<ScratchCardMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, handle_mouse_input)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ScratchCardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    // Load static assets
    let reveal = asset_server.load("images/prize.png");
    let star = asset_server.load("images/star_pattern.png");

    // Create initial black mask (0 = show cover layer star pattern)
    let initial_mask_data = vec![0u8; 512 * 512];
    let initial_mask = images.add(create_mask_image(&initial_mask_data, 512));

    // Store resource information
    commands.insert_resource(ScratchResources {
        reveal_texture: reveal.clone(),
        cover_texture: star.clone(),
        mask_data: initial_mask_data,
        size: 512,
    });

    // Create initial material with correct mask
    let material = materials.add(ScratchCardMaterial {
        reveal_texture: reveal,
        scratch_mask: initial_mask,
        cover_layer: star,
    });

    // Scene setup
    commands.spawn(Camera2d::default());
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::from_size(Vec2::splat(512.0)))),
        MeshMaterial2d(material),
        Transform::from_xyz(0.0, 0.0, 0.0),
        ScratchQuad, // Mark this quad for easy material updates
    ));
}

/// Component to mark the scratch card quad
#[derive(Component)]
struct ScratchQuad;

/// Handle mouse input and update material
fn handle_mouse_input(
    windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut scratch_res: ResMut<ScratchResources>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ScratchCardMaterial>>,
    mut quad_query: Query<&mut MeshMaterial2d<ScratchCardMaterial>, With<ScratchQuad>>,
) {
    if !buttons.pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };

    let Ok((camera, cam_tf)) = camera_q.single() else {
        return;
    };

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    // Convert coordinates
    if let Ok(world) = camera.viewport_to_world(cam_tf, cursor_pos) {
        let pos = world.origin.truncate();
        let uv = (pos + Vec2::splat(256.0)) / 512.0;

        if uv.x >= 0.0 && uv.x <= 1.0 && uv.y >= 0.0 && uv.y <= 1.0 {
            // Update mask data
            let size = scratch_res.size as i32;
            let px = (uv.x * size as f32) as i32;
            let py = ((1.0 - uv.y) * size as f32) as i32;

            let mut changed = false;

            for dy in -BRUSH_RADIUS..=BRUSH_RADIUS {
                for dx in -BRUSH_RADIUS..=BRUSH_RADIUS {
                    if dx * dx + dy * dy > BRUSH_RADIUS * BRUSH_RADIUS {
                        continue;
                    }
                    let x = px + dx;
                    let y = py + dy;
                    if x >= 0 && x < size && y >= 0 && y < size {
                        let idx = (y * size + x) as usize;
                        if scratch_res.mask_data[idx] != 255 {
                            scratch_res.mask_data[idx] = 255;
                            changed = true;
                        }
                    }
                }
            }

            if changed {
                // println!("Updating mask at UV: {:?}", uv); // Optional debug output

                // Create new mask image
                let new_mask =
                    images.add(create_mask_image(&scratch_res.mask_data, scratch_res.size));

                // Create new material
                let new_material = materials.add(ScratchCardMaterial {
                    reveal_texture: scratch_res.reveal_texture.clone(),
                    scratch_mask: new_mask,
                    cover_layer: scratch_res.cover_texture.clone(),
                });

                // Update quad's material
                if let Ok(mut material_handle) = quad_query.single_mut() {
                    material_handle.0 = new_material;
                }
            }
        }
    }
}

/// Create mask image
fn create_mask_image(data: &[u8], size: u32) -> Image {
    let mut img = Image::new(
        Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data.to_vec(),
        TextureFormat::R8Unorm,
        RenderAssetUsages::all(),
    );

    img.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST;
    img.sampler = ImageSampler::nearest();
    img
}
