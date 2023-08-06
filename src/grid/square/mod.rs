use bevy::{prelude::*, sprite::{MaterialMesh2dBundle}};

pub const SQUARE_LENGTH: i32 = 8;

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

    let length: f32 = SQUARE_LENGTH as f32;

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Box::new(length,length,length).into()).into(),
            material: materials.add(ColorMaterial::from(color)),
            transform: Transform::from_translation(Vec3::new(pos_x as f32, pos_y as f32, 0.)),
            ..default()
    }, Square));
}
