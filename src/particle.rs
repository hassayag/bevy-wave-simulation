use std::f32::consts::TAU;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use geo_types::coord;
use geo::{Line, Coord};
use geo::line_intersection::{line_intersection, LineIntersection};

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
    pos: Vec2,
    velocity: Vec2,
    time_remaining: f32,
    time_to_collision: f32,
    collision_checked: bool,
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
    }, Particle{ pos: Vec2::new(pos.x, pos.y), velocity, time_remaining: LIFE_SECS, time_to_collision: 0., collision_checked: false}));
}


fn update(
    mut query_particles: Query<(&mut Particle, &mut Transform, &mut Handle<ColorMaterial>, Entity), With<Particle>>,
    query_obstacles: Query<&Obstacle>,
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
            
        let move_this_frame = Vec2::new(
            particle.velocity.x * SPEED * time.delta_seconds(),
            particle.velocity.y * SPEED * time.delta_seconds(),
        );

        transform.translation.x += move_this_frame.x;
        transform.translation.y += move_this_frame.y;
        
        let mut collision_pos = Vec2::new(0.,0.);
        let mut found_collision = false;

        if !particle.collision_checked {
            match predict_collision_pos(&particle, &obstacles[0]) {
                Some(pos) => {
                    collision_pos = pos;
                    found_collision = true;
                    println!("Intersection at {collision_pos}");
                }
                None => {}
            }
        }

        particle.collision_checked = true;

        if found_collision {
            particle.time_to_collision = (collision_pos.x - particle.pos.x) / particle.velocity.x; 
        }

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

fn predict_collision_pos(particle: &Particle, obstacle: &Obstacle) -> Option<Vec2> {
    let particle_trajectory = Line::new(
        coord!{x:particle.pos.x, y:particle.pos.y},
        coord!{x:particle.pos.x + 1000.*particle.velocity.x, y:particle.pos.y + 1000.*particle.velocity.y},
    );

    let obstacle_line = Line::new(
        coord!{x:obstacle.v1.x, y:obstacle.v1.y},
        coord!{x:obstacle.v2.x, y:obstacle.v2.y},
    );

    // let expected = LineIntersection::SinglePoint { intersection: coord! { x: 2.5, y: 2.5 }, is_proper: true };
    let intersection_option = line_intersection(particle_trajectory, obstacle_line);

    let mut single_point: Vec2 = Vec2::new(0., 0.);
    let mut found_intersection = false;

    match intersection_option {
        None => println!("Failed"),
        Some(intersection) => {
            match intersection {
                LineIntersection::SinglePoint { intersection, is_proper } => {
                    single_point = Vec2::new(intersection.x, intersection.y);
                    found_intersection = true;
                }
        
                LineIntersection::Collinear { intersection } => {
                    // Handle the Collinear variant here if needed
                    println!("Collinear Intersection: {:?}", intersection);
                }
            }        
        }
    }

    if found_intersection {
        Some(single_point)
    }
    else {
        None
    }
}
