use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub const LENGTH: i32 = 4;

#[derive(Component)]
struct Square;

pub fn spawn(
    pos: Vec3,
    color: Color,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let length: f32 = LENGTH as f32;

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Box::new(length,length,length).into()).into(),
            material: materials.add(ColorMaterial::from(color)),
            transform: Transform::from_translation(Vec3::new(pos.x, pos.y, pos.z)),
            ..default()
    }, Square));
}
