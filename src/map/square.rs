use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use crate::map;

#[derive(Component)]
struct Square;

pub fn spawn(
    pos: Vec3,
    size: Vec3,
    color: Color,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Box::new(size.x, size.y, size.z).into()).into(),
            material: materials.add(ColorMaterial::from(color)),
            transform: Transform::from_translation(Vec3::new(pos.x, pos.y, pos.z)),
            ..default()
    }, Square));
}
