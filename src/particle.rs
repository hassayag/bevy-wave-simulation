use std::f32::consts::TAU;

use bevy::sprite::{Mesh2dHandle, Material2d};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use geo_types::coord;
use geo::Line;
use geo::line_intersection::{line_intersection, LineIntersection};

use crate::map::Obstacle;

const RADIUS: f32 = 2.5;
const SPEED: f32 = 100.;
const NUM_OF_PARTICLES: usize = 10000;
const LIFE_SECS: f32 = 10.;
const COLLISSION_LIFE_LOSS_PERC: f32 = 0.5;
const COLLISSION_SPEED_LOSS_PERC: f32 = 0.0;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init)
            .add_systems(Update, update);
    }
}

#[derive(Component)]
struct Particle {
    pos: Vec2,
    velocity: Vec2,
    time_remaining: f32,
    time_to_collision: f32,
    collision_pos: Vec2,
    rebound_dir: Vec2,
    collision_checked: bool,
}

#[derive(Resource)]
pub struct ParticleMesh {
    pub bundle: MaterialMesh2dBundle<ColorMaterial>,
}

#[derive(Resource)]
struct State {
    last_num_of_obstacles: usize
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.insert_resource(State{ last_num_of_obstacles: 0 });
    commands.insert_resource(ParticleMesh {
        bundle: MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        } 
    })
}


pub fn spawn_wave(
    pos: Vec3,
    commands: &mut Commands,
    bundle: &MaterialMesh2dBundle<ColorMaterial>,
) {
    for i in 0..NUM_OF_PARTICLES {
        let angle = i as f32 * TAU / NUM_OF_PARTICLES as f32;
        let velocity = Vec2::new(f32::cos(angle), f32::sin(angle));

        commands.spawn((bundle.clone(), 
            Particle { 
                pos: Vec2::new(pos.x, pos.y), 
                velocity, 
                time_remaining: LIFE_SECS, 
                time_to_collision: 0., 
                collision_pos: Vec2::ZERO,
                rebound_dir: Vec2::ZERO,
                collision_checked: false
            }));
    }
}

// fn spawn(
//     pos: Vec3,
//     velocity: Vec2,
//     commands: &mut Commands,
//     particle_mesh: &Res<ParticleMesh>,
// ) {
//     commands.spawn((particle_mesh.bundle, 
//         Particle { 
//             pos: Vec2::new(pos.x, pos.y), 
//             velocity, 
//             time_remaining: LIFE_SECS, 
//             time_to_collision: 0., 
//             collision_pos: Vec2::ZERO,
//             rebound_dir: Vec2::ZERO,
//             collision_checked: false
//         }));
// }


fn update(
    mut state: ResMut<State>,
    mut query_particles: Query<(&mut Particle, &mut Transform, &mut Handle<ColorMaterial>, Entity), With<Particle>>,
    query_obstacles: Query<&Obstacle>,
    mut commands: Commands,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut num_of_obstacles = 0;
    for obstacle in query_obstacles.iter() {
        num_of_obstacles += 1;
    }

    for (
        mut particle, 
        mut transform, 
        mut material, 
        entity
    ) in query_particles.iter_mut() {       
        // find next collision if we have not checked
        // OR if the number of obstacles has changed
        if !particle.collision_checked || state.last_num_of_obstacles != num_of_obstacles {
            state.last_num_of_obstacles = num_of_obstacles;
            for obstacle in query_obstacles.iter() {
                let mut found_collision = false;
                let mut collision_pos = Vec2::ZERO;
                
                match predict_collision_pos(&particle, obstacle) {
                    Some(pos) => {
                        collision_pos = pos;
                        found_collision = true;
                    }
                    None => {}
                }

                if found_collision  {
                    let time_to_collision = ((collision_pos.x - particle.pos.x) / (SPEED * particle.velocity.x)).abs();
                    
                    // find minimum time_to_collision
                    if time_to_collision < particle.time_to_collision || particle.time_to_collision == 0. {
                        particle.time_to_collision = time_to_collision;

                        particle.rebound_dir = obstacle.normal;
                        particle.collision_pos = collision_pos;
                    }
                }
            }

            particle.collision_checked = true;
        }
        

        // check if we have collided
        if particle.time_to_collision > 0. {
            particle.time_to_collision -= time.delta_seconds();
        }
        
        let mut move_time = time.delta_seconds();

        // handle collision
        if particle.time_to_collision < 0. {
            particle.velocity = reflect_about_normal(particle.velocity, particle.rebound_dir);

            // place particle exactly at collision point
            particle.pos = particle.collision_pos;
            transform.translation.x = particle.collision_pos.x;
            transform.translation.y = particle.collision_pos.y;

            // account for the wasted time
            move_time -= particle.time_to_collision;

            // reset collision data
            particle.time_to_collision = 0.;
            particle.collision_checked = false;
            particle.rebound_dir = Vec2::ZERO;
        }

        let move_this_frame = Vec2::new(
            particle.velocity.x * SPEED * move_time,
            particle.velocity.y * SPEED * move_time,
        );

        transform.translation.x += move_this_frame.x;
        transform.translation.y += move_this_frame.y;
        particle.pos.x = transform.translation.x;
        particle.pos.y = transform.translation.y;

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
        None => {},
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

fn reflect_about_normal(d: Vec2, n: Vec2) -> Vec2 {
    return d - 2. * (d.dot(n)) * n;
}