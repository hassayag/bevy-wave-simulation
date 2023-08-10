use bevy::prelude::*;
pub mod square;
use crate::map;

pub const SQUARE_LENGTH: i32 = 16*50;
pub const SIZE: Dimension = Dimension{x:1, y:1};
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
    pub x: i32,
    pub y: i32
}


fn init(    
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    build_grid((SIZE.x, SIZE.y), (0, 0), &mut commands, &mut meshes, &mut materials);
    build_obstacles(&mut commands, &mut meshes, &mut materials);
}

fn build_grid(
    dimensions: (i32, i32), offset: (i32, i32),
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>) {
    
    let (dim_x, dim_y) = dimensions;
    let (offset_x, offset_y) = offset;

    let min_x = offset_x;
    let max_x = dim_x + offset_x;

    let min_y = offset_y;
    let max_y = dim_y + offset_y;

    for i in min_x .. max_x {
        for j in min_y .. max_y {
            let color = Color::Rgba { 
                red: divide_ints(i,dim_x), 
                green: divide_ints(j,dim_y), 
                blue: divide_ints(i+j,2*dim_y),
                alpha: 1. 
            };
            let len = map::SQUARE_LENGTH as f32;
            let pos = Vec3::new(i as f32*len + len/2.,j as f32*len + len/2., 0.);

            let length: f32 = map::SQUARE_LENGTH as f32;
            square::spawn(pos, Vec3::new(length,length,length), color, commands, meshes, materials);
        }
    }
    
}

fn build_obstacles(    
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>
) {
    let len = map::SQUARE_LENGTH as f32;
    let color = Color::ORANGE;

    square::spawn(Vec3::new(len/2., len/2., 0.05), Vec3::new(len/4.,len/4.,len/4.), color, commands, meshes, materials);
}

fn divide_ints(a: i32, b: i32) -> f32 {
    return a as f32 / b as f32;
}
