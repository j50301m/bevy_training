use bevy::{prelude::*, sprite::Material2dPlugin};
mod mask_2d;
use mask_2d::Mask2DMaterial;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                watch_for_changes_override: Some(true),
                ..default()
            }),
            Material2dPlugin::<Mask2DMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<Mask2DMaterial>>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(Camera2d::default());

    let main_tex = asset_server.load("images/main.png");
    let mask_tex = asset_server.load("images/mask.png");

    let material = materials.add(Mask2DMaterial {
        main_texture: main_tex,
        mask_texture: mask_tex,
    });

    let mesh_handle = meshes.add(Rectangle::from_size(Vec2::splat(200.0)));
    commands.spawn((
        Mesh2d(mesh_handle),
        MeshMaterial2d(material),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
