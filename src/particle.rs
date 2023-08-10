use std::f32::consts::TAU;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use crate::map;

const RADIUS: f32 = 2.5;
const SPEED: f32 = 80.;
const NUM_OF_PARTICLES: usize = 1500;
const LIFE_SECS: f32 = 40.;
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
    mut query: Query<(&mut Particle, &mut Transform, &mut Handle<ColorMaterial>, Entity), With<Particle>>,
    mut commands: Commands,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (mut particle, mut transform, mut material, entity) in query.iter_mut() {
        let (x_collision, y_collision) = check_boundary(&transform.translation);

        let mut move_this_frame = Vec2::new(
            particle.velocity.x * SPEED * time.delta_seconds(),
            particle.velocity.y * SPEED * time.delta_seconds(),
        );

        // if we have collided AND the velocity towards the boundary, reverse the velocity
        if x_collision != 0 && particle.velocity.x * x_collision as f32 > 0. {
            let diff_x = transform.translation.x - ((x_collision + 1)/ 2 * map::ACTUAL_SIZE.x) as f32; 

            // the amount a particle would move through the boundary during the collision frame
            let wasted_x = diff_x - move_this_frame.x;

            // subtract 2 x waste to compensate the change in direction at the boundary
            move_this_frame.x -= 2.*wasted_x;

            // reverse and reduce velocity
            particle.velocity.x = -1. * (1. - COLLISSION_SPEED_LOSS_PERC) * particle.velocity.x;
            particle.time_remaining = particle.time_remaining * (1. - COLLISSION_LIFE_LOSS_PERC);
        }
        if y_collision != 0 && particle.velocity.y * y_collision as f32 > 0. {
            let diff_y = transform.translation.y - ((y_collision + 1)/ 2 * map::ACTUAL_SIZE.y) as f32; 

            // the amount a particle would move through the boundary during the collision frame
            let wasted_y = diff_y - move_this_frame.y;

            // subtract 2 x waste to compensate the change in direction at the boundary
            move_this_frame.y -= 2.*wasted_y;
            
            // reverse and reduce velocity
            particle.velocity.y = -1. * (1. - COLLISSION_SPEED_LOSS_PERC) * particle.velocity.y;
            particle.time_remaining = particle.time_remaining * (1. - COLLISSION_LIFE_LOSS_PERC);
        }

        transform.translation.x += move_this_frame.x;
        transform.translation.y += move_this_frame.y;
        
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
fn check_boundary(pos: &Vec3) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;

    if pos.x > map::ACTUAL_SIZE.x as f32{
        x = 1;
    }
    else if pos.x < 0. {
        x = -1;
    }

    if pos.y > map::ACTUAL_SIZE.y as f32{
        y = 1;
    }
    else if pos.y < 0. {
        y = -1;
    }

    return (x, y);
}
