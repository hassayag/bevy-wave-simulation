use bevy::prelude::*;

mod grid;
use grid::GridPlugin;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);



fn main() {
    App::new()
        .add_systems(Startup, setup_camera)
        .add_plugins((DefaultPlugins, GridPlugin))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
