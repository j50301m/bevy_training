use bevy::{
    prelude::*, render::render_resource::{AsBindGroup, ShaderRef}, sprite::{Material2d, Material2dPlugin}
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CircleMaskMaterial {
}

impl Material2d for CircleMaskMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/circle_mask.wgsl".into()
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                watch_for_changes_override: Some(true),
                ..default()
            }),
            Material2dPlugin::<CircleMaskMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut  meshs: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CircleMaskMaterial>>,
) {
    commands.spawn(Camera2d::default());

    let mesh_handle = meshs.add(Rectangle::from_size(Vec2::splat(200.0)));
    commands.spawn((
        Mesh2d(mesh_handle),
        MeshMaterial2d(materials.add(CircleMaskMaterial{})), // radius is set to 100.0
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
