use bevy::{
    asset::RenderAssetUsages, 
    image::ImageSampler, 
    prelude::*, 
    render::render_resource::{AsBindGroup, Extent3d, ShaderRef, TextureDimension, TextureFormat, TextureUsages}, 
    sprite::{AlphaMode2d, Material2d, Material2dPlugin}, 
    window::PrimaryWindow,
};

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct ScratchCardMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub reveal_texture: Handle<Image>,
    #[texture(2)]
    #[sampler(3)]
    pub scratch_mask: Handle<Image>, // The mask to control cover-layer's transparency
    #[texture(4)]
    #[sampler(5)]
    pub cover_layer: Handle<Image>, // The cover layer texture
}

impl Material2d for ScratchCardMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/scratch_card.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        ShaderRef::Default
    }

    // fn alpha_mode(&self) -> AlphaMode2d {
    //     AlphaMode2d::Blend
    // }
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // 保持紋理像素感
        .add_plugins(Material2dPlugin::<ScratchCardMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, paint_mask_with_mouse)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ScratchCardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // -- 1. 載入資源
    let reveal = asset_server.load("images/prize.png");
    let star   = asset_server.load("images/star_pattern.png");
    let mask_handle = images.add(empty_mask_image());

    // -- 2. 建立 Material
    let material = materials.add(ScratchCardMaterial {
        reveal_texture: reveal,
        scratch_mask: mask_handle.clone(),
        cover_layer: star,
    });

    // -- 3. 畫面
    commands.spawn(Camera2d::default());
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::from_size(Vec2::splat(512.0)))),
        MeshMaterial2d(material),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // -- 4. 把 mask handle 存資源，好讓 paint_system 用
    commands.insert_resource(MaskHandle(mask_handle));
}



/// 產生 512×512 的全黑遮罩
fn empty_mask_image() -> Image {
    let size = Extent3d { width: 512, height: 512, depth_or_array_layers: 1 };
    
    // 創建簡單的測試圖案：左上角白色，其他黑色
    let mut data = vec![0u8; 512 * 512];
    for y in 0..256 {
        for x in 0..256 { 
            let idx = y * 512 + x;
            data[idx] = 255; // 左上角設為白色
        }
    }
    
    let mut img = Image::new(
        size, 
        TextureDimension::D2, 
        data,
        TextureFormat::R8Unorm,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD
    );
    
    // 允許 CPU → GPU 複製
    img.texture_descriptor.usage = TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING;
    // 不要預設 linear filter，保持硬邊
    img.sampler = ImageSampler::nearest();
    img
}


/// 資源：遮罩的 Handle
#[derive(Resource)]
struct MaskHandle(Handle<Image>);

/// -------- 滑鼠塗抹系統 --------
fn paint_mask_with_mouse(
    windows: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut images: ResMut<Assets<Image>>,
    mask_handle: Res<MaskHandle>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    if !buttons.pressed(MouseButton::Left) {
        return;
    }
    
    // 获取窗口
    let Ok(window) = windows.get_single() else {
        return;
    };
    
    // 获取相机
    let Ok((camera, cam_tf)) = camera_q.get_single() else {
        return;
    };
    
    // 获取鼠标位置
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    
    println!("Cursor position: {:?}", cursor_pos);

    // 1. 將螢幕座標 → 世界座標 → UV(0~1)
    if let Ok(world) = camera.viewport_to_world(cam_tf, cursor_pos) {
        let pos = world.origin.truncate();
        let uv  = (pos + Vec2::splat(256.0)) / 512.0; // quad 是 512x512 且居中
        if uv.x.is_nan() || uv.y.is_nan() { return; }

        // 2. 映射到像素
        if let Some(img) = images.get_mut(&mask_handle.0) {
            let w = img.texture_descriptor.size.width  as i32;
            let h = img.texture_descriptor.size.height as i32;
            let px = (uv.x * w as f32) as i32;
            let py = ((1.0 - uv.y) * h as f32) as i32; // flip Y

            println!("UV: {:?}, Pixel: ({}, {})", uv, px, py);

            // 3. 畫一個半徑 12px 的圓
            let radius = 50;
            let data = img.data.as_mut().unwrap();
            let mut changed = false;
            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    if dx*dx + dy*dy > radius*radius { continue; }
                    let x = px + dx;
                    let y = py + dy;
                    if x >= 0 && x < w && y >= 0 && y < h {
                        let idx = (y * w + x) as usize;
                        if data[idx] != 255 {
                            data[idx] = 255; // 白 = 顯示獎品
                            changed = true;
                        }
                    }
                }
            }
            
            // 重要：通知 Bevy 纹理已经改变
            if changed {
                println!("Texture updated! Changed pixels in mask.");
            }
        }
    }
}