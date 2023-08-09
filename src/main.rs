use bevy::prelude::*;

pub mod grid;
mod camera;
mod input;
mod particle;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins, 
            grid::GridPlugin, 
            camera::CameraPlugin, 
            input::InputPlugin,
            particle::ParticlePlugin
        ))
        .run();
}
