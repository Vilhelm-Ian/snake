use bevy::{
    prelude::*, sprite::collide_aabb::collide, sprite::MaterialMesh2dBundle,
    time::common_conditions::on_timer, utils::Duration, window::PresentMode,
};
use rand::prelude::*;
use std::io;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;
const TIME_STEP: f32 = 1. / 9.;

#[derive(Copy, Clone)]
struct Cordinates {
    x: usize,
    y: usize,
}

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
        .insert_resource(FixedTime::new_from_secs(10.0))
        .add_systems(Startup, setup)
        //.add_system(movement)
        .add_system(movement.run_if(on_timer(Duration::from_secs_f32(TIME_STEP))))
        .add_system(move_body_parts)
        .insert_resource(FixedTime::new_from_secs(5.0))
        .add_system(key_press)
        //.add_system(grid_name_later)
        .add_system(check_for_collisions)
        .run();

    let row = vec![" "; WIDTH];
    let mut grid = vec![row; HEIGHT];
    add_fruit(&mut grid);
    let mut x: i32 = 5;
    let mut y: i32 = 5;
    let last_cordinate_of_tail = Cordinates { x: 0, y: 0 };
    let mut body_part_cordinates = vec![Cordinates {
        x: x as usize,
        y: y as usize,
    }];
    loop {
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed = input_text.trim();
        let movement = match trimmed.parse::<char>() {
            Ok(i) => i,
            Err(..) => 'x',
        };
        let mut vertical_direction: i32 = 0;
        let mut horizontal_direction: i32 = 0;
        match movement {
            'l' => horizontal_direction += 1,
            'h' => horizontal_direction -= 1,
            'k' => vertical_direction -= 1,
            'j' => vertical_direction += 1,
            _ => continue,
        };
        grid[y as usize][x as usize] = " ";
        y += vertical_direction;
        x += horizontal_direction;
        if y >= grid.len() as i32 {
            y = 0;
        }
        if y < 0 {
            y = grid.len() as i32 - 1;
        }
        if x >= grid[0].len() as i32 {
            x = 0;
        }
        if x < 0 {
            x = grid[0].len() as i32 - 1;
        }
        if grid[y as usize][x as usize] == "O" {
            add_fruit(&mut grid);
            body_part_cordinates.push(last_cordinate_of_tail);
        }
        if grid[y as usize][x as usize] == "#" {
            println!("you lose");
            break;
        }
        grid[y as usize][x as usize] = "#";
        update_body_parts(
            &mut body_part_cordinates,
            Cordinates {
                x: x as usize,
                y: y as usize,
            },
            last_cordinate_of_tail,
        );
        add_snake_to_grid(&body_part_cordinates, &mut grid);
        print_grid(&grid);
    }
}

#[derive(Component)]
struct Head {
    x: i32,
    y: i32,
    last_cordinate_of_tail: Cordinates,
    body_part_cordinates: Vec<Cordinates>,
}

#[derive(Component)]
struct Tail;
#[derive(Component)]
struct Collider;

#[derive(Component)]
struct BodyPart {
    id: usize,
}

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
        Collider,
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
        Head {
            x: 0,
            y: 0,
            last_cordinate_of_tail: Cordinates { x: 0, y: 0 },
            body_part_cordinates: vec![],
        },
    ));
}

fn movement(
    _fixed_time: Res<FixedTime>,
    mut query: Query<(&mut Transform, &mut Head)>,
    _body_part_query: Query<(&mut Transform, &BodyPart)>,
) {
    for (mut body_part, mut head) in query.iter_mut() {
        head.last_cordinate_of_tail = Cordinates {
            x: body_part.translation.x as usize,
            y: body_part.translation.y as usize,
        };
        body_part.translation.x += head.x as f32 * 50.0;
        if body_part.translation.x >= WIDTH as f32 * 50.0 {
            body_part.translation.x = 0.0;
        }
        if body_part.translation.x < 0.0 {
            body_part.translation.x = 50.0 * WIDTH as f32 - 50.0;
        }
        body_part.translation.y += head.y as f32 * 50.0;
        if body_part.translation.y >= HEIGHT as f32 * 50.0 {
            body_part.translation.y = 0.0;
        }
        if body_part.translation.y < 0.0 {
            body_part.translation.y = 50.0 * HEIGHT as f32 - 50.0;
        }
    }
}

fn move_body_parts(
    head_query: Query<&Transform, With<Head>>,
    mut body_part_query: Query<(&mut Transform, &BodyPart)>,
) {
    let mut body_parts = vec![];
    let mut how_many_body_parts = 0;
    for (_, body_part) in body_part_query.iter() {
        if body_part.id > how_many_body_parts {
            how_many_body_parts = body_part.id;
        }
    }
    for (transform, body_part) in body_part_query.iter_mut() {
        if body_parts.len() == 0 {
            body_parts = vec![
                Transform {
                    translation: Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    rotation: Quat::from_array([1.0, 1.0, 1.0, 1.0]),
                    scale: Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                };
                how_many_body_parts
            ];
        }
        body_parts[body_part.id].translation.x = transform.translation.x;
        body_parts[body_part.id].translation.y = transform.translation.y;
    }
    for index in (1..body_parts.len()).rev() {
        body_parts[index] = body_parts[index - 1];
    }
    let head = head_query.single();
    body_parts[0].translation.x = head.translation.x;
    body_parts[0].translation.y = head.translation.y;
}

fn key_press(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Head, With<Head>)>,
    _time_step: Res<FixedTime>,
) {
    for (_transform, mut head, _) in query.iter_mut() {
        if head.x == 0 {
            if keyboard_input.just_pressed(KeyCode::Left) {
                head.y = 0;
                head.x = -1;
            }
            if keyboard_input.just_pressed(KeyCode::Right) {
                head.x = 1;
                head.y = 0;
            }
        }
        if head.y == 0 {
            if keyboard_input.just_pressed(KeyCode::Up) {
                head.y = 1;
                head.x = 0;
            }
            if keyboard_input.just_pressed(KeyCode::Down) {
                head.y = -1;
                head.x = 0;
            }
        }
    }
}

fn grid_name_later(mut query: Query<&Transform, &Head>) {
    let row = vec![" "; WIDTH];
    let mut grid = vec![row; HEIGHT];
    for head in query.iter_mut() {
        grid[(head.translation.y / 10.0) as usize][(head.translation.x / 10.0) as usize] = "#";
    }
    print_grid(&grid);
}

fn check_for_collisions(
    mut commands: Commands,
    collider_query: Query<(&Transform, Entity), With<Collider>>,
    mut head_query: Query<(&Transform, &mut Head), With<Head>>,
    body_part_query: Query<Entity, With<BodyPart>>,
) {
    let (head_transform, head) = head_query.single_mut();
    let head_size = head_transform.scale.truncate();
    let mut number_of_body_parts = 0;
    for _ in body_part_query.iter() {
        number_of_body_parts += 1;
    }

    for _ in &collider_query {
        for (transform, entity) in collider_query.iter() {
            let collision = collide(
                head_transform.translation,
                head_size,
                transform.translation,
                transform.scale.truncate(),
            );
            if let Some(_collision) = collision {
                println!("collide");
                commands.entity(entity).despawn();
                // head.body_part_cordinates.push(head.last_cordinate_of_tail);

                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(0.25, 0.25, 0.75),
                            custom_size: Some(Vec2::new(50.0, 50.0)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(
                            head.last_cordinate_of_tail.y as f32,
                            head.last_cordinate_of_tail.x as f32,
                            0.,
                        )),
                        ..default()
                    },
                    Tail,
                    BodyPart {
                        id: number_of_body_parts + 1,
                    },
                ));
            }
        }
    }
}

fn print_grid(grid: &Vec<Vec<&str>>) {
    for row in grid {
        println!("{:?}", row);
    }
}

fn find_empty_places(grid: &Vec<Vec<&str>>) -> Vec<Cordinates> {
    let mut result = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == " " {
                result.push(Cordinates { x, y })
            }
        }
    }
    result
}

fn add_fruit(grid: &mut Vec<Vec<&str>>) {
    let mut rng = thread_rng();
    let empty_places = find_empty_places(grid);
    let random_square = rng.gen_range(0..empty_places.len());
    let place_to_add_fruit = empty_places[random_square];
    grid[place_to_add_fruit.y][place_to_add_fruit.x] = "O";
}

fn update_body_parts(
    body_parts: &mut Vec<Cordinates>,
    new_direction: Cordinates,
    mut last_cordinate_of_tail: Cordinates,
) {
    last_cordinate_of_tail = body_parts[body_parts.len() - 1].clone();
    for iter in (0..body_parts.len()).rev() {
        if iter == 0 {
            body_parts[iter].x = new_direction.x;
            body_parts[iter].y = new_direction.y;
        } else {
            body_parts[iter] = body_parts[iter - 1];
        }
    }
}

fn add_snake_to_grid(body_parts: &Vec<Cordinates>, grid: &mut Vec<Vec<&str>>) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == "#" {
                grid[y][x] = " ";
            }
        }
    }
    for cordinate in body_parts {
        println!("{:?} {:?}", cordinate.y, cordinate.x);
        grid[cordinate.y][cordinate.x] = "#"
    }
}
