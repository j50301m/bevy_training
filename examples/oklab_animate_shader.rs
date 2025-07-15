use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
};

const OKLAB_SHADER_PATH: &str = "shaders/oklab_animate_shader.wgsl";
const SHADER_PATH: &str = "shaders/animate_shader.wgsl";

#[derive(Clone, Copy, ShaderType, Debug, Resource)]
struct Params {
    phase: f32,
    speed: f32,
    _pad: Vec2, // 16‑byte alignment
}

#[derive(Component)]
struct SpeedUI;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct OklabMaterial {
    #[uniform(0)]
    pub params: Params,
}

impl Material for OklabMaterial {
    fn fragment_shader() -> ShaderRef {
        OKLAB_SHADER_PATH.into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct NormalMaterial {
    #[uniform(0)]
    pub params: Params,
}

impl Material for NormalMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialPlugin::<OklabMaterial>::default())
        .add_plugins(MaterialPlugin::<NormalMaterial>::default())
        .insert_resource(Params {
            phase: 0.0,
            speed: 1.0,
            _pad: Vec2::ZERO,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_keys, accumulate_phase,update_materials))
        .add_systems(Update, update_ui)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<OklabMaterial>>,
    mut normal_materials: ResMut<Assets<NormalMaterial>>,
) {
    // Spawn a  simple cube with the oklab material
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(OklabMaterial {
            params: Params {
                phase: 0.0,
                speed: 1.0,
                _pad: Vec2::ZERO,
            },
        })),
        Transform::from_xyz(-2.0, 0.0, 0.0),
    ));

    // Spawn a simple cube with the normal material
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(normal_materials.add(NormalMaterial {
            params: Params {
                phase: 0.0,
                speed: 1.0,
                _pad: Vec2::ZERO,
            },
        })),
        Transform::from_xyz(2.0, 0.0, 0.0),
    ));

    // Add a camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 4.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Add a user interface to display instructions
    commands.spawn((
        Text::new("Use Arrow Up/Down to adjust speed, R to reset."),
        TextFont::default(),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));

    commands.spawn((
        SpeedUI,
        Text::new("Speed:"),
        TextFont::default(),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(30.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));
}

fn handle_keys(
    input: Res<ButtonInput<KeyCode>>,
    mut params: ResMut<Params>,
    // mut materials: ResMut<Assets<OklabMaterial>>,
) {
    if input.just_pressed(KeyCode::ArrowUp) {
        params.speed = (params.speed + 0.1).min(5.0);
    } else if input.just_pressed(KeyCode::ArrowDown) {
        params.speed = (params.speed - 0.1).max(0.1);
    } else if input.just_pressed(KeyCode::KeyR) {
        params.speed = 1.0;
    }

    // for (_, mat) in materials.iter_mut() {
    //     let mut speed = mat.params.speed;
    //     if input.just_pressed(KeyCode::ArrowUp) {
    //         speed += 0.1;
    //     } else if input.just_pressed(KeyCode::ArrowDown) {
    //         speed -= 0.1;
    //     }
    //     mat.params.speed = speed.clamp(0.1, 5.0);
    // }
}

fn accumulate_phase(
    mut params: ResMut<Params>,
    time: Res<Time>,
    // mut materials: ResMut<Assets<OklabMaterial>>,
) {
    let dt = time.delta_secs();

    // Accumulate the phase based on the time delta
    params.phase += dt * params.speed;

    // // Ensure the phase stays within [0, 2π]
    // for (_, mat) in materials.iter_mut() {
    //     mat.params.phase += dt * mat.params.speed;

    //     // Ensure the phase stays within [0, 2π]
    //     if mat.params.phase > std::f32::consts::TAU {
    //         mat.params.phase -= std::f32::consts::TAU;
    //     }
    // }
}

fn update_ui(
    parms: Res<Params>,
    ui_root: Single<Entity, (With<Text>, With<SpeedUI>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*ui_root, 0) = format!("Speed: {:.1}, Phase: {:.2}", parms.speed, parms.phase);
}


fn update_materials(
    params: Res<Params>,
    mut oklab_mat: ResMut<Assets<OklabMaterial>>,
    mut normal_mat: ResMut<Assets<NormalMaterial>>,
){
    for (_, mat) in oklab_mat.iter_mut() {
        mat.params = *params;
    }

    for (_, mat) in normal_mat.iter_mut() {
        mat.params = *params;
    }
}