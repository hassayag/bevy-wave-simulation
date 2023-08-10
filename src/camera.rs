use bevy::prelude::*;
use bevy::core_pipeline::clear_color::ClearColorConfig;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init)
            .add_systems(Update, keyboard_input);
    }
}

#[derive(Component)]
pub struct MainCamera;

fn init(mut commands: Commands) {
    commands.spawn((Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgb(0.06, 0.06, 0.06)),
        },
        ..Default::default()
    }, MainCamera));
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