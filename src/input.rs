use bevy::{prelude::*, window::PrimaryWindow};
use crate::{camera, particle};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_input);
    }
}

fn mouse_input(    
    buttons: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<camera::MainCamera>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (camera, camera_transform) = camera_q.single();

    let mouse_pos: Vec2;

    if let Some(position) = q_windows.single().cursor_position()
    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
    .map(|ray| ray.origin.truncate()) {
        mouse_pos = position;
    } else {
        return;
    }
    
    if buttons.just_pressed(MouseButton::Left) {
        println!("Click at {}", mouse_pos);
        let pos = Vec3::new(mouse_pos.x, mouse_pos.y, 0.001);

        particle::spawn_wave(pos, Color::WHITE, &mut commands, &mut meshes, &mut materials);
    }
}
