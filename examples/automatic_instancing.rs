use bevy::{
    prelude::*,
    render::{mesh::MeshTag, render_resource::AsBindGroup},
};

const SHADER_PATH: &str = "shaders/automatic_instancing.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[texture(0)]
    pub texture: Handle<Image>,
}

impl Material for CustomMaterial {
    fn vertex_shader() -> bevy::render::render_resource::ShaderRef {
        SHADER_PATH.into()
    }
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        SHADER_PATH.into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }
}

#[derive(Resource)]
struct LoadingImage {
    is_loaded: bool,
    handle: Handle<Image>,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialPlugin::<CustomMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (setup_texture, animate))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(LoadingImage {
        is_loaded: false,
        handle: asset_server.load("images/ferris.png"),
    });

    // Spawn a camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, -5.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn setup_texture(
    mut commands: Commands,
    mut loading_image: ResMut<LoadingImage>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    images: Res<Assets<Image>>,
) {
    if loading_image.is_loaded
        || !asset_server
            .load_state(loading_image.handle.id())
            .is_loaded()
    {
        return;
    }

    loading_image.is_loaded = true;

    // Create a mesh
    let mesh = meshes.add(Cuboid::from_size(Vec3::splat(0.01)));

    // Create a material with the texture
    let material_handle = materials.add(CustomMaterial {
        texture: loading_image.handle.clone(),
    });

    // Get the dimensions of the loaded image
    let loaded_image = images.get(&loading_image.handle).unwrap();
    let image_dims = UVec2::new(loaded_image.width(), loaded_image.height());
    let total_pixels = image_dims.x * image_dims.y;

    for index in 0..total_pixels {
        let x = index % image_dims.x;
        let y = index / image_dims.y;

        // Convert to centered world coordinates
        let world_x = (x as f32 - image_dims.x as f32 / 2.0) * 0.01;
        let world_y = -(y as f32 - image_dims.y as f32 / 2.0) * 0.01;

        commands.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material_handle.clone()),
            MeshTag(index),
            Transform::from_xyz(world_x, world_y, 0.0),
        ));
    }
}

fn animate(
    time: Res<Time>,
    mut transforms: Query<(&mut Transform, &MeshTag)>,
    loading_image: Res<LoadingImage>,
) {
    if !loading_image.is_loaded {
        return;
    }

    let time_secs = time.elapsed_secs();

    for (mut transform, _mesh_tag) in transforms.iter_mut() {
        // Wave effect parameters
        let wave_amplitude = 1.0; // Wave amplitude
        let wave_frequency = 2.0; // Wave frequency
        let wave_speed = 2.0; // Wave speed

        // Create wave effect - based on X position and time
        let wave_offset = (transform.translation.x * wave_frequency + time_secs * wave_speed).sin()
            * wave_amplitude;

        // Apply wave to Z axis (depth)
        transform.translation.z = (wave_offset).sin() * wave_amplitude;

        // Optional: Add rotation effect to make wave more lively
        let rotation_angle = wave_offset * 0.2; // Slight rotation
        transform.rotation = Quat::from_rotation_z(rotation_angle);
    }
}
