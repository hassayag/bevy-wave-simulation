use bevy::prelude::*;

pub mod map;
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
            map::MapPlugin, 
            camera::CameraPlugin, 
            input::InputPlugin,
            particle::ParticlePlugin
        ))
        .run();
}
