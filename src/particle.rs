use std::f32::consts::TAU;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use crate::grid::square;
use crate::grid;

const SPEED: f32 = 80.;
const NUM_OF_PARTICLES: usize = 500;
const LIFE_SECS: f32 = 15.;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update);
    }
}

#[derive(Component)]
struct Particle {
    velocity: Vec2,
    time_remaining: f32,
}

pub fn spawn_wave(
    pos: Vec3,
    color: Color,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>
) {
    for i in 0..NUM_OF_PARTICLES {
        let angle = i as f32 * TAU / NUM_OF_PARTICLES as f32;
        let velocity = Vec2::new(f32::cos(angle), f32::sin(angle));
        spawn(pos, velocity, color, commands, meshes, materials);
    }
}

fn spawn(
    pos: Vec3,
    velocity: Vec2,
    color: Color,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let length: f32 = square::LENGTH as f32;

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(length).into()).into(),
            material: materials.add(ColorMaterial::from(color)),
            transform: Transform::from_translation(Vec3::new(pos.x, pos.y, pos.z)),
            ..default()
    }, Particle{velocity, time_remaining: LIFE_SECS}));
}


fn update(
    mut query: Query<(&mut Particle, &mut Transform, Entity), With<Particle>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (mut particle, mut transform, entity) in query.iter_mut() {
        let (x_collision, y_collision) = check_boundary(&transform.translation);

        if x_collision {
            particle.velocity.x = -1. * particle.velocity.x;
        }
        if y_collision {
            particle.velocity.y = -1. * particle.velocity.y;
        }
        
        transform.translation.x += particle.velocity.x * SPEED * time.delta_seconds();
        transform.translation.y += particle.velocity.y * SPEED * time.delta_seconds();

        particle.time_remaining -= time.delta_seconds();

        if particle.time_remaining < 0. {
            commands.entity(entity).despawn();
        }
    }
}

/**
 * For each X and Y axes, returns whether it has collided with the boundary
 */
fn check_boundary(pos: &Vec3) -> (bool, bool) {
    let mut x = false;
    let mut y = false;

    if pos.x > grid::ACTUAL_SIZE.x as f32{
        x = true;
    }
    else if pos.x < 0. {
        x = true;
    }

    if pos.y > grid::ACTUAL_SIZE.y as f32{
        y = true;
    }
    else if pos.y < 0. {
        y = true;
    }

    return (x, y);
}

