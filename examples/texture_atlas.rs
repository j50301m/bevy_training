use bevy::{color::palettes::css::YELLOW, prelude::*, winit::WinitSettings};

#[derive(Resource)]
struct AnimateSpeed(f32);

#[derive(Component)]
struct SpeedText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(AnimateSpeed(5.0))
        .insert_resource(WinitSettings::game())
        .add_systems(Startup, setup)
        .add_systems(Update, (player_input, update_speed_ui))
        .add_systems(FixedUpdate, image_animation)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load a texture atlas
    let texture_handle = asset_server.load("images/mani-idle-run.png");
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // 主要佈局容器
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            display: Display::Flex,
            ..default()
        })
        .with_children(|parent| {
            // 文字區域
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(80.0), // 固定高度給文字
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Start,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                })
                .with_children(|text_parent| {
                    text_parent.spawn((
                        Text::new("Use Arrow Up/Down to adjust speed."),
                        TextFont::default(),
                    ));

                    text_parent
                        .spawn((Text::new("Now Speed: "), TextFont::default()))
                        .with_children(|parent| {
                            parent.spawn((
                                TextSpan::new("5.0"),
                                TextFont::default(),
                                TextColor(YELLOW.into()),
                                SpeedText,
                            ));
                        });
                });

            // 動畫區域
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0), // 剩餘空間
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|anim_parent| {
                    anim_parent.spawn((
                        ImageNode::from_atlas_image(
                            texture_handle,
                            TextureAtlas::from(texture_atlas_handle),
                        ),
                        Node {
                            width: Val::Px(100.0),
                            height: Val::Px(100.0),
                            ..default()
                        },
                    ));
                });
        });
    commands.spawn(Camera2d);
}

fn image_animation(
    time: Res<Time>,
    animate_speed: Res<AnimateSpeed>,
    mut query: Query<&mut ImageNode>,
) {
    for mut image_node in query.iter_mut() {
        // 檢查是否有 TextureAtlas
        if let Some(ref mut atlas) = image_node.texture_atlas {
            // 根據時間計算當前幀
            let frame_time = 1.0 / animate_speed.0; // 每幀的時間
            let elapsed = time.elapsed_secs();

            // 計算當前應該顯示第幾幀（0-6，因為有7幀）
            let current_frame = ((elapsed / frame_time) as usize) % 7;

            atlas.index = current_frame;
        }
    }
}

fn player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut animate_speed: ResMut<AnimateSpeed>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        animate_speed.0 = (animate_speed.0 + 1.0).min(20.0);
    }
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        animate_speed.0 = (animate_speed.0 - 1.0).max(1.0);
    }
}

fn update_speed_ui(
    animate_speed: Res<AnimateSpeed>,
    mut query: Query<&mut TextSpan, With<SpeedText>>,
) {
    for mut text in query.iter_mut() {
        text.0 = format!("{:.1}", animate_speed.0);
    }
}
