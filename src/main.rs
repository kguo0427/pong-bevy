use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, ( spawn_ball, spawn_camera, spawn_paddles ))
        .add_systems(Update, (move_ball, project_positions.after(move_ball)))
        .run();
}

#[derive(Component, Default)]
#[require(Transform)]
struct Position(Vec2);

#[derive(Component, Default)]
struct Velocity(Vec2);

#[derive(Component)]
#[require(Position, Velocity = Velocity(Vec2::new(-1., 1.)))]
struct Ball;

#[derive(Component)]
#[require(Position)]
struct Paddle;

const BALL_SIZE: f32 = 5.;
const PADDLE_WIDTH: f32 = 10.;
const PADDLE_HEIGHT: f32 = 50.;
const BALL_SPEED: f32 = 5.;

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(( Camera2d,
        Transform::from_xyz(0., 0., 0.)));
}
fn spawn_ball(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>){
    println!("spawning ball");
    let shape = Circle::new(BALL_SIZE);
    let color = Color::srgb(1., 0., 0.);
    
    let mesh = meshes.add(shape);
    let material = materials.add(color);
    commands.spawn(( Ball, Mesh2d(mesh), MeshMaterial2d(material) ));
}

fn move_ball(mut ball: Query<( &mut Position, &Velocity ), With<Ball>>){
   if let Ok((mut position, velocity)) = ball.single_mut(){
       position.0 += velocity.0 * BALL_SPEED
   } 
}
fn project_positions(mut positionables: Query<( &mut Transform, &Position )>){
    for (mut transform, position) in &mut positionables{
        transform.translation = position.0.extend(0.);
    }
}
fn spawn_paddles(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>){
    println!("spawning paddles");
    let shape = Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT);
    let color = Color::srgb(0., 1., 0.);
    
    let mesh = meshes.add(shape);
    let material = materials.add(color);
    
    commands.spawn(( Paddle, Mesh2d(mesh), MeshMaterial2d(material), Position(Vec2::new(25., 0.))));
}