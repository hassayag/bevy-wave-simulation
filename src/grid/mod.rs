use bevy::prelude::*;
mod square;

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
    build_grid((200, 200), (0, 0), &mut commands, &mut meshes, &mut materials);
}

fn build_grid(
    dimensions: (i32, i32), offset: (i32, i32),
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>) {
    
    let (dim_x, dim_y) = dimensions;
    let (offset_x, offset_y) = offset;

    for i in offset_x .. dim_x + offset_x {
        for j in offset_y .. dim_y + offset_y {
            let color = Color::Rgba { 
                red: (i * 255/dim_x) as f32, 
                green: (j * 255/dim_y) as f32, 
                blue: (i/(j+1) * 255) as f32,
                alpha: 255.0 
            };
            
            info!("{color:?}");
            square::spawn_square((i,j), color, commands, meshes, materials);
        }
    }
}
