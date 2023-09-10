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
    // build_obstacles(&mut commands, &mut meshes, &mut materials);
}

#[derive(Component)]
pub struct Obstacle {
    // defines two ends of the 1-D obstacle
    pub v1: Vec2,
    pub v2: Vec2,
    pub normal: Vec2,
}

pub fn create_obstacle(
    v1: Vec2, v2: Vec2,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>
) {
    let color = Color::ORANGE;

    let dir = v2 - v1;
    let pos = v1 + dir/2.;
    let pos3 = Vec3::new(pos.x,pos.y,0.);
    let size = Vec3::new(SQUARE_LENGTH/100., dir.length(), 0.1);
    let rot: f32 = (dir.y/dir.x).atan() + PI/2.;
    
    let normal_dir: Vec3 = cross_product(Vec3::new(v2.x-pos.x, v2.y-pos.y, 0.), Vec3::new(0.,0.,1.));
    // let normal_dir: Vec3 = cross_product(Vec3::new(1.,0.,0.), Vec3::new(0.,1.,0.));

    square::spawn(
        pos3,
        size, 
        rot,
        Obstacle {v1, v2, normal: Vec2::new(normal_dir.x, normal_dir.y)},
        color, commands, meshes, materials
    );

    // println!("NORMAL {normal_dir}");
    // square::debug(Vec3::new(pos.x, pos.y, 0.5), commands, meshes, materials);
    // square::debug(Vec3::new(v1.x, v1.y, 0.5), commands, meshes, materials);
    // square::debug(Vec3::new(v2.x, v2.y, 0.5), commands, meshes, materials);
    // square::debug(pos3 + 20.*normal_dir + Vec3::new(0.,0.,0.5), commands, meshes, materials);
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
    return Vec3 {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }.normalize()
}