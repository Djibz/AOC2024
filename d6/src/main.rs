use std::fs;

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Top,
    Bottom,
    Right,
    Left,
}

fn next_position(position: (i32, i32), direction: &Direction) -> (i32, i32) {
    let (x, y) = position;
    return match direction {
        Direction::Top => (x, y - 1),
        Direction::Bottom => (x, y + 1),
        Direction::Right => (x + 1, y),
        Direction::Left => (x - 1, y),
    };
}

fn next_direction(direction: Direction) -> Direction {
    return match direction {
        Direction::Top => Direction::Right,
        Direction::Bottom => Direction::Left,
        Direction::Right => Direction::Bottom,
        Direction::Left => Direction::Top,
    };
}

fn main() {
    println!("Day 6");
    let file_path = "./input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut current_position: (i32, i32) = (0, 0);

    let mut direction: Direction = Direction::Top;

    let mut obstacles: Vec<(i32, i32)> = vec![];

    let mut height: i32 = 0;
    let mut width: i32 = 0;

    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '^' {
                current_position = (x.try_into().unwrap(), y.try_into().unwrap());
            }
            if c == '#' {
                obstacles.push((x.try_into().unwrap(), y.try_into().unwrap()));
            }
            width = (x + 1).try_into().unwrap();
        }
        height = (y + 1).try_into().unwrap();
    }

    let starting_position: (i32, i32) = current_position;
    println!("Starting position : {:?}", starting_position);

    let mut went: Vec<(i32, i32)> = vec![];

    loop {
        went.push(current_position);
        let next = next_position(current_position, &direction);
        let (x, y) = next;

        if x < 0 || y < 0 || y >= height || x >= width {
            break;
        }

        if obstacles.contains(&next) {
            direction = next_direction(direction);
        } else {
            current_position = next;
        }
    }

    went.sort();
    went.dedup();

    println!("Part 1 : {:?}", went.len());

    let mut total = 0;

    for a in 0..(width) {
        println!("{}/{}", a, width);
        for b in 0..(height) {
            if obstacles.contains(&(a, b)) {
                continue;
            }
            let mut path: Vec<((i32, i32), Direction)> = vec![];

            obstacles.push((a, b));
            current_position = starting_position;
            let mut direction: Direction = Direction::Top;

            loop {
                path.push((current_position, direction.clone()));
                let next = next_position(current_position, &direction);
                let (x, y) = next;

                if x < 0 || y < 0 || y >= height || x >= width {
                    break;
                }

                if obstacles.contains(&next) {
                    direction = next_direction(direction);
                } else {
                    current_position = next;
                    if path.contains(&(current_position, direction.clone())) {
                        total += 1;
                        break;
                    }
                }
            }

            obstacles.pop();
        }
    }

    println!("Part 2 : {:?}", total);
}
