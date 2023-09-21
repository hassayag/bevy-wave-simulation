use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
pub struct Square;

pub fn spawn<T>(
    pos: Vec3,
    size: Vec3,
    rot: f32,
    component: T,
    color: Color,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) 
    where
        T: Component,
{
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Box::new(size.x, size.y, size.z).into()).into(),
            material: materials.add(ColorMaterial::from(color)),
            transform: Transform {
                translation: Vec3::new(pos.x, pos.y, pos.z),
                rotation: Quat::from_rotation_z(rot),
                scale: Vec3::new(1.,1.,1.)
            },
            ..default()
    }, component));
}


pub fn debug(
    pos: Vec3,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) 
{
    commands.spawn(
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(3.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform {
                translation: Vec3::new(pos.x, pos.y, pos.z),
                rotation: Quat::from_rotation_z(0.),
                scale: Vec3::new(1.,1.,1.)
            },
            ..default()
    });
}