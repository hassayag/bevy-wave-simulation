use std::f32::consts::PI;

use bevy::prelude::*;
pub mod square;
use crate::map;

pub const SQUARE_LENGTH: f32 = 16.*50.;
pub const SIZE: Dimension = Dimension{x:1., y:1.};
pub const ACTUAL_SIZE: Dimension = Dimension {
    x: SIZE.x * SQUARE_LENGTH,
    y: SIZE.y * SQUARE_LENGTH,
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init);
    }
}

pub struct Dimension {
    pub x: f32,
    pub y: f32
}


fn init(    
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    build_obstacles(&mut commands, &mut meshes, &mut materials);
}

#[derive(Component)]
pub struct Obstacle {
    // defines two ends of the 1-D obstacle
    pub v1: Vec2,
    pub v2: Vec2
}

fn build_obstacles(    
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>
) {
    let len = 40.;
    let color = Color::ORANGE;

    let pos = Vec3::new(SQUARE_LENGTH/2., SQUARE_LENGTH/2., 0.05);
    let size = Vec3::new(SQUARE_LENGTH/50., SQUARE_LENGTH/10., len/4.);
    let rot: f32 = PI/6.;

    let v1 = Vec2::new(pos.x - size.y/2. * rot.sin(), pos.y - size.y/2. * rot.cos());
    let v2 = Vec2::new(pos.x + size.y/2. * rot.sin(), pos.y + size.y/2. * rot.cos());

    println!("V1 {} V2 {}", v1, v2);

    println!("Obstacle {} {}", pos, size);
    square::spawn(
        pos,
        size, 
        -1.*rot,
        Obstacle {v1, v2},
        color, commands, meshes, materials
    );
}

fn divide_ints(a: i32, b: i32) -> f32 {
    return a as f32 / b as f32;
}
