use bevy::{
    prelude::*, sprite::collide_aabb::collide, sprite::MaterialMesh2dBundle,
    time::common_conditions::on_timer, utils::Duration, window::PresentMode,
};
use rand::prelude::*;
use std::io;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "I am a window!".into(),
                present_mode: PresentMode::AutoVsync,
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(FixedTime::new_from_secs(1.0 / 10.0))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (despawn_head, spawn_head, update_cordinates)) //change to fixed update later
        .add_systems(Update, (movement)) //change to fixed update later
        .run();
}

#[derive(Component)]
struct Ball {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct BodyPart {
    id: usize,
}
#[derive(Component, Clone, Copy)]
struct Cordinates {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Direction {
    x: i32,
    y: i32,
}

#[derive(Event)]
struct BallEaten;

#[derive(Event, Default)]
struct CollisionEvent;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle { ..default() });

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(25.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(150., 150., 0.)),
            ..default()
        },
        Ball { x: 3, y: 3 },
    ));
    // geerate grid
    for x in 0..WIDTH {
        commands.spawn((SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(3.0, 1000.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(x as f32 * 50.0, 0., 0.)),
            ..default()
        },));
    }
    // Rectangle
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(50., 50., 0.)),
            ..default()
        },
        BodyPart { id: 0 },
    ));
    commands.spawn((Cordinates { x: 1, y: 1 }, Direction { x: 0, y: 0 }));
}

fn movement(mut cordinates: Query<(&mut Direction)>, keyboard_input: Res<Input<KeyCode>>) {
    let mut direction = cordinates.single_mut();
    if keyboard_input.just_pressed(KeyCode::Left) && direction.x != 1 {
        direction.x = -1;
        direction.y = 0;
    }
    if keyboard_input.just_pressed(KeyCode::Right) && direction.x != -1 {
        direction.x = 1;
        direction.y = 0;
    }
    if keyboard_input.just_pressed(KeyCode::Up) && direction.y != -1 {
        direction.y = 1;
        direction.x = 0;
    }
    if keyboard_input.just_pressed(KeyCode::Down) && direction.y != 1 {
        direction.y = -1;
        direction.x = 0;
    }
}

fn despawn_head(
    mut commands: Commands,
    mut body_part_query: Query<(Entity, &mut BodyPart)>,
    ball_query: Query<(&Ball, Entity)>,

    cordinates_query: Query<&Cordinates>,
) {
    let (ball, ball_entity) = ball_query.single();
    let cordinates = cordinates_query.single();
    if ball.x == cordinates.x && ball.y == cordinates.y {
        commands.entity(ball_entity).despawn();
        return;
    }

    for (body_entity, mut body_part) in body_part_query.iter_mut() {
        if body_part.id == 0 {
            commands.entity(body_entity).despawn();
            println!("despawning")
        } else {
            body_part.id -= 1;
        }
    }
}

fn spawn_head(
    mut commands: Commands,
    body_part_query: Query<(Entity, &BodyPart)>,
    cordinates_query: Query<&mut Cordinates>,
) {
    let cordinates = cordinates_query.single();
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                50. * cordinates.x as f32,
                50. * cordinates.y as f32,
                0.,
            )),
            ..default()
        },
        BodyPart {
            id: body_part_query.iter().len() - 1,
        },
    ));
}

fn update_cordinates(mut cordinates_query: Query<(&mut Cordinates, &Direction)>) {
    let (mut cordinates, direction) = cordinates_query.single_mut();
    cordinates.x += direction.x;
    cordinates.y += direction.y;
    if cordinates.y > HEIGHT as i32 {
        cordinates.y = 0;
    }
    if cordinates.y < 0 {
        cordinates.y = HEIGHT as i32;
    }
    if cordinates.x > WIDTH as i32 {
        cordinates.x = 0;
    }
    if cordinates.x < 0 {
        cordinates.x = WIDTH as i32;
    }
}

fn ball_eaten(
    mut commands: Commands,
    ball_query: Query<(&Ball, Entity)>,
    cordinates_query: Query<&Cordinates>,
    body_part_query: Query<(&Transform, With<BodyPart>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (ball, ball_entity) = ball_query.single();
    let cordinates = cordinates_query.single();
    if ball.x == cordinates.x && ball.y == cordinates.y {
        commands.entity(ball_entity).despawn();
        let empty_cordinates = get_empty_cordinates(body_part_query);
        let mut rng = thread_rng();
        let n: usize = rng.gen_range(0..empty_cordinates.len());
        let cordinate = empty_cordinates[n];

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(25.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(Vec3::new(
                    50. * cordinate.x as f32,
                    50. * cordinate.y as f32,
                    0.,
                )),
                ..default()
            },
            Ball {
                x: cordinate.x,
                y: cordinate.y,
            },
        ));
    }
}

fn get_empty_cordinates(body_part_query: Query<(&Transform, With<BodyPart>)>) -> Vec<Cordinates> {
    let mut cordinates = vec![];
    for y in 0..HEIGHT {
        cordinates.push(vec![]);
        for x in 0..WIDTH {
            let len = cordinates.len();
            cordinates[len - 1].push(Cordinates {
                x: x as i32,
                y: y as i32,
            });
        }
    }
    for (body_part, _) in body_part_query.iter() {
        let y = body_part.translation.y as usize;
        let x = body_part.translation.x as usize;
        cordinates[y][x] = Cordinates { x: -1, y: -1 }
    }
    let result: Vec<Cordinates> = cordinates
        .into_iter()
        .flatten()
        .filter(|cordinate| cordinate.x != -1)
        .collect();
    result
}
