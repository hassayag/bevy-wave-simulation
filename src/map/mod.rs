use std::f32::consts::PI;

use bevy::prelude::*;
pub mod square;

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
    pub v2: Vec2,
    pub normal: Vec2,
}

fn build_obstacles(    
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>
) {
    let len = 40.;
    let color = Color::ORANGE;

    let pos = Vec3::new(SQUARE_LENGTH/2., SQUARE_LENGTH/2., 0.05);
    let size = Vec3::new(SQUARE_LENGTH/50., SQUARE_LENGTH, len/4.);
    let rot: f32 = PI/6.;

    let v1 = Vec2::new(pos.x - size.y/2. * rot.sin(), pos.y - size.y/2. * rot.cos());
    let v2 = Vec2::new(pos.x + size.y/2. * rot.sin(), pos.y + size.y/2. * rot.cos());

    let normal_to_obstacle = cross_product(Vec3::new(v2.x-v1.x, v2.y-v1.y, 0.), Vec3::new(0., 0., 1.) - pos);

    square::spawn(
        pos,
        size, 
        -1.*rot,
        Obstacle {v1, v2, normal: Vec2::new(normal_to_obstacle.x, normal_to_obstacle.y)},
        color, commands, meshes, materials
    );

    let pos = Vec3::new(SQUARE_LENGTH/2. + 200., SQUARE_LENGTH/2.-100., 0.05);
    let size = Vec3::new(SQUARE_LENGTH/50., SQUARE_LENGTH, len/4.);
    let rot: f32 = PI/6.;

    let v1 = Vec2::new(pos.x - size.y/2. * rot.sin(), pos.y - size.y/2. * rot.cos());
    let v2 = Vec2::new(pos.x + size.y/2. * rot.sin(), pos.y + size.y/2. * rot.cos());

    let normal_to_obstacle = cross_product(Vec3::new(v2.x-v1.x, v2.y-v1.y, 0.), Vec3::new(0., 0., 1.) - pos);

    square::spawn(
        pos,
        size, 
        -1.*rot,
        Obstacle {v1, v2, normal: Vec2::new(normal_to_obstacle.x, normal_to_obstacle.y)},
        color, commands, meshes, materials
    );
}

fn cross_product(a: Vec3, b: Vec3) -> Vec3 {
    let c: [[f32; 3]; 3] = [[0., -a.z, a.y],
                            [a.z, 0., -a.x], 
                            [-a.y, a.x, 0.]];

    let cross_prod =  Vec3::new(
        c[0][0] * b.x + c[0][1] * b.x + c[0][2] * b.x,
        c[1][0] * b.y + c[1][1] * b.y + c[1][2] * b.y,
        c[2][0] * b.z + c[2][1] * b.z + c[2][2] * b.z,
    );
 
    let normalised_cross_prod = cross_prod / (cross_prod.x.powi(2) + cross_prod.y.powi(2) + cross_prod.z.powi(2)).sqrt();

    return normalised_cross_prod;
}
