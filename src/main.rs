use bevy::prelude::*;
use bevy::math::bounding::{
    Aabb2d,
    BoundingCircle,
    BoundingVolume,
    IntersectsVolume
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, ( spawn_ball, spawn_camera, spawn_paddles ))
        .add_systems(Update, (move_ball, project_positions.after(move_ball), handle_collisions.after(move_ball)))
        .run();
}

#[derive(Component, Default)]
#[require(Transform)]
struct Position(Vec2);

#[derive(Component, Default)]
struct Velocity(Vec2);

#[derive(Component, Default)]
struct Shape(Vec2);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Component)]
#[require(
    Position, 
    Velocity = Velocity(Vec2::new(-1., 0.)),
    Shape = Shape(Vec2::new(BALL_SIZE, BALL_SIZE))
)]
struct Ball;

#[derive(Component)]
#[require(
    Position,
    Velocity,
    Shape = Shape(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT))
)]
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
    
    commands.spawn(( Paddle, Mesh2d(mesh.clone()), MeshMaterial2d(material.clone()), Position(Vec2::new(500., 0.))));
    commands.spawn(( Paddle, Mesh2d(mesh.clone()), MeshMaterial2d(material.clone()), Position(Vec2::new(-500., 0.))));
}

fn collide_with_side(ball: BoundingCircle, wall: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&wall){
        return None;
    }

    let closest = wall.closest_point(ball.center());
    let offset = ball.center() - closest;
    let side = if offset.x.abs() > offset.y.abs(){
        if offset.x < 0. {
            Collision::Left
        }  else{
            Collision::Right
        }
    } else{
        if offset.y > 0.{
            Collision::Top
        } else {
            Collision::Bottom
        }
    };
    Some(side)
}

fn handle_collisions( mut ball: Query<(&mut Velocity, &Position, &Shape), With<Ball>>, paddles: Query<( &Position, &Shape ), With<Paddle>>){
    if let Ok((mut ball_velocity, ball_position, ball_shape)) = ball.single_mut() {
        for (position, shape) in &paddles{
            if let Some(collision) = collide_with_side(
                BoundingCircle::new(ball_position.0, ball_shape.0.x),
                Aabb2d::new(position.0, shape.0/2.)
            ) {
                if collision == Collision::Left || collision == Collision::Right {
                    ball_velocity.0.x *= -1.;
                }
                if collision == Collision::Top || collision == Collision::Bottom {
                    ball_velocity.0.y *= -1.;
                }
            }
        }
    }
}