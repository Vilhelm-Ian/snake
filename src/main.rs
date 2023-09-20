use rand::prelude::*;
use std::io;

#[derive(Copy, Clone)]
struct Cordinates {
    x: usize,
    y: usize,
}

fn main() {
    let mut row = vec![" "; 10];
    let mut grid = vec![row; 10];
    add_fruit(&mut grid);
    let mut x: i32 = 5;
    let mut y: i32 = 5;
    let mut last_cordinate_of_tail = Cordinates { x: 0, y: 0 };
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
            _ => (),
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
