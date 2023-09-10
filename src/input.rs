use bevy::{prelude::*, window::PrimaryWindow};
use crate::{camera, particle::{self, ParticleMesh}, map::{self}};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init)
           .add_systems(Update, mouse_input);
    }
}

#[derive(Resource)]
struct InputState {
    last_key_right_click: bool, 
    last_right_click_pos: Vec2,
}

fn init(
    mut commands: Commands,
) {
    commands.insert_resource(InputState { last_key_right_click: false, last_right_click_pos: Vec2::ZERO})
}

fn mouse_input(
    mut state: ResMut<InputState>,
    buttons: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<camera::MainCamera>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    particle_mesh: Res<ParticleMesh>,
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
        let pos = Vec3::new(mouse_pos.x, mouse_pos.y, 0.001);
        particle::spawn_wave(pos, &mut commands, &particle_mesh.bundle);
    }

    if buttons.just_pressed(MouseButton::Right) && state.last_key_right_click {
        map::create_obstacle(mouse_pos, state.last_right_click_pos, &mut commands, &mut meshes, &mut materials);
        state.last_key_right_click = false;
    }
    else if buttons.just_pressed(MouseButton::Right) {
        state.last_key_right_click = true;
        state.last_right_click_pos = mouse_pos;
    }
}
