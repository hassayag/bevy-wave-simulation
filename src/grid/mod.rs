use bevy::prelude::*;
mod square;
use square::*;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup);
            // .add_systems(Update, update);
    }
}

fn startup(    
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    build_grid((50, 50), (0, 0), &mut commands, &mut meshes, &mut materials);
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
            // info!("{color:?}");
            square::spawn_square((i*SQUARE_LENGTH,j*SQUARE_LENGTH), color, commands, meshes, materials);
        }
    }
}

fn divide_ints(a: i32, b: i32) -> f32 {
    return a as f32 / b as f32;
}
