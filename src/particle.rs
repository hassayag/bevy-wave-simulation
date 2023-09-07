use std::f32::consts::TAU;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use crate::map::{self, Obstacle};

const RADIUS: f32 = 2.5;
const SPEED: f32 = 100.;
const NUM_OF_PARTICLES: usize = 1;
const LIFE_SECS: f32 = 20.;
const COLLISSION_LIFE_LOSS_PERC: f32 = 0.5;
const COLLISSION_SPEED_LOSS_PERC: f32 = 0.0;

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
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(color)),
            transform: Transform::from_translation(Vec3::new(pos.x, pos.y, pos.z)),
            ..default()
    }, Particle{velocity, time_remaining: LIFE_SECS}));
}


fn update(
    mut query_particles: Query<(&mut Particle, &mut Transform, &mut Handle<ColorMaterial>, Entity), With<Particle>>,
    mut query_obstacles: Query<&Obstacle>,
    mut commands: Commands,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut obstacles: Vec<&Obstacle> = Vec::new();
    for obstacle in query_obstacles.iter() {
        obstacles.push(obstacle);
    }

    for (
        mut particle, 
        mut transform, 
        mut material, 
        entity
    ) in query_particles.iter_mut() {
            

        // transform.translation.x += move_this_frame.x;
        // transform.translation.y += move_this_frame.y;
        
        // reduce opacity of particle each loop
        let new_material = materials.add(ColorMaterial::from(Color::Rgba { 
                red: 1., green: 1., blue: 1., 
                alpha: particle.time_remaining / LIFE_SECS 
            }));

        // Update the material handle
        *material = new_material.clone();

        particle.time_remaining -= time.delta_seconds();
        
        if particle.time_remaining < 0. {
            commands.entity(entity).despawn();
        }
    }
}

/**
 * For each X and Y axes, returns if +ve or -ve boundary was crossed
 */
fn check_boundary(pos: &Vec3, obstacles: &Vec<&Obstacle>) {
}
