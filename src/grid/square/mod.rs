use std::f32::consts::TAU;
use bevy::{prelude::*, sprite::{MaterialMesh2dBundle}};

// pub struct SquarePlugin;

// impl Plugin for SquarePlugin {
//     fn build(&self, app: &mut App) {
//         app
//             .add_systems(Startup, startup)
//             .add_systems(Update, update);
//     }
// }

#[derive(Component)]
struct Square;

pub fn spawn_square(
    pos: (i32, i32),
    color: Color,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let (pos_x, pos_y) = pos;

    // Circle
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Box::new(1.,1., 1.).into()).into(),
            material: materials.add(ColorMaterial::from(color)),
            transform: Transform::from_translation(Vec3::new(pos_x as f32, pos_y as f32, 0.)),
            ..default()
    }, Square));
}

// const VELOCITY: Vec3 = Vec3::new(10.,10., 0.);

// fn update(mut query: Query<&mut Transform, With<Square>>, time: Res<Time>) {
//     for mut transform in &mut query {
//         transform.rotate_z(0.1 * TAU * time.delta_seconds());
//         transform.translation += VELOCITY * time.delta_seconds();
//     }
// }