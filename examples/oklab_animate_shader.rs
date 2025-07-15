use bevy::{ prelude::*, render::render_resource::{AsBindGroup, ShaderRef}};


const SHADER_PATH: &str = "shaders/oklab_animate_shader.wgsl";


#[derive(Asset,TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial{
    #[uniform(0)]
    pub speed: f32,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_PATH.into()
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialPlugin::<CustomMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, change_speed)
        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {

    // Spawn a simple cube with the custom material
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(CustomMaterial {
            speed: 1.0,
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Add a camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 4.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y)
    ));

}

fn change_speed(
    input: Res<ButtonInput<KeyCode>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    if input.just_pressed(KeyCode::ArrowUp) {
        // Increase speed
        for (_, material) in materials.iter_mut() {
            material.speed += 0.1;
            material.speed = material.speed.min(5.0); // Max limit
            println!("Speed increased to: {:.1}", material.speed);
        }
    } else if input.just_pressed(KeyCode::ArrowDown) {
        // Decrease speed
        for (_, material) in materials.iter_mut() {
            material.speed -= 0.1;
            material.speed = material.speed.max(0.1); // Min limit
            println!("Speed decreased to: {:.1}", material.speed);
        }
    } else if input.just_pressed(KeyCode::KeyR) {
        // Reset
        for (_, material) in materials.iter_mut() {
            material.speed = 1.0;
            println!("Speed reset to: 1.0");
        }
    }
}