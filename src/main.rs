use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PresentMode};
use rand::prelude::*;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "I am a window!".into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(FixedTime::new_from_secs(1.0 / 10.0))
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(Cordinates { x: 1, y: 1 })
        .insert_resource(Direction { x: 0, y: 0 })
        .insert_resource(GameOver { state: false })
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (despawn_head, ball_eaten, spawn_head, update_cordinates),
        ) //change to fixed update later
        .add_systems(Update, (movement, check_collision, update_scoreboard)) //change to fixed update later
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
#[derive(Resource, Clone, Copy)]
struct Cordinates {
    x: i32,
    y: i32,
}

#[derive(Resource)]
struct Direction {
    x: i32,
    y: i32,
}
#[derive(Event)]
struct BallEaten;

#[derive(Resource)]
struct GameOver {
    state: bool,
}

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Resource)]
struct Scoreboard {
    score: usize,
}

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

    // Scoreboard
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        }),
    );
}

fn movement(mut direction: ResMut<Direction>, keyboard_input: Res<Input<KeyCode>>) {
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

    ball_query: Query<&Ball>,
    cordinates: Res<Cordinates>,
    game_over: Res<GameOver>,
) {
    if game_over.state {
        return;
    }
    let ball = ball_query.single();
    if ball.x == cordinates.x && ball.y == cordinates.y {
        return;
    }
    for (body_entity, mut body_part) in body_part_query.iter_mut() {
        if body_part.id == 0 {
            commands.entity(body_entity).despawn();
        } else {
            body_part.id -= 1;
        }
    }
}

fn spawn_head(
    mut commands: Commands,
    body_part_query: Query<(Entity, &BodyPart)>,
    cordinates: Res<Cordinates>,
    game_over: Res<GameOver>,
) {
    if game_over.state {
        return;
    }
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

fn update_cordinates(mut cordinates: ResMut<Cordinates>, direction: Res<Direction>) {
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
    cordinates: Res<Cordinates>,
    body_part_query: Query<&Transform, With<BodyPart>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut scoreboard: ResMut<Scoreboard>,
) {
    let (ball, ball_entity) = ball_query.single();
    if ball.x == cordinates.x && ball.y == cordinates.y {
        commands.entity(ball_entity).despawn();
        let empty_cordinates = get_empty_cordinates(body_part_query);
        let mut rng = thread_rng();
        let n: usize = rng.gen_range(0..empty_cordinates.len());
        let cordinate = empty_cordinates[n];
        scoreboard.score += 1;

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

fn get_empty_cordinates(body_part_query: Query<&Transform, With<BodyPart>>) -> Vec<Cordinates> {
    let mut cordinates = vec![];
    for y in 0..=HEIGHT {
        cordinates.push(vec![]);
        for x in 0..=WIDTH {
            let len = cordinates.len();
            cordinates[len - 1].push(Cordinates {
                x: x as i32,
                y: y as i32,
            });
        }
    }
    for body_part in body_part_query.iter() {
        let y = (body_part.translation.y / 50.) as usize;
        let x = (body_part.translation.x / 50.) as usize;
        cordinates[y][x] = Cordinates { x: -1, y: -1 }
    }
    let result: Vec<Cordinates> = cordinates
        .into_iter()
        .flatten()
        .filter(|cordinate| cordinate.x != -1)
        .collect();
    result
}

fn check_collision(
    body_part_query: Query<(&BodyPart, &Transform)>,
    mut game_over: ResMut<GameOver>,
) {
    let len = body_part_query.into_iter().len();
    for (body_part, transform) in body_part_query.iter() {
        if body_part.id == len - 1 {
            let head = transform.translation;
            for (body_part, transform) in body_part_query.iter() {
                if body_part.id != len - 1 && transform.translation == head {
                    game_over.state = true;
                }
            }
            break;
        }
    }
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}
