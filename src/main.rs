use bevy::prelude::*;

mod grid;
use grid::GridPlugin;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);



fn main() {
    App::new()
        .add_systems(Startup, setup_camera)
        .add_plugins((DefaultPlugins, GridPlugin))
        .add_systems(Update, keyboard_input)
        .run();
}


#[derive(Component)]
struct MainCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<MainCamera>>,
    time: Res<Time>,
) {
    const CAMERA_SPEED: f32 = 100.;
    
    let mut camera = query.single_mut();
    
    if keys.pressed(KeyCode::W) {
        camera.translation.y += CAMERA_SPEED * time.delta_seconds();
    }
    if keys.pressed(KeyCode::A) {
        camera.translation.x -= CAMERA_SPEED * time.delta_seconds();
    }
    if keys.pressed(KeyCode::S) {
        camera.translation.y -= CAMERA_SPEED * time.delta_seconds();
    }
    if keys.pressed(KeyCode::D) {
        camera.translation.x += CAMERA_SPEED * time.delta_seconds();
    }
}
