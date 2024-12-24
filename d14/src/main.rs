use regex::Regex;
use std::fs;

fn move_robot(
    index: usize,
    positions: &mut Vec<(u32, u32)>,
    speeds: &Vec<(i32, i32)>,
    width: u32,
    height: u32,
) {
    let (x, y) = positions[index];
    let (sx, sy) = speeds[index];

    positions[index] = (
        ((width as i32 + x as i32 + sx) % width as i32) as u32,
        ((height as i32 + y as i32 + sy) % height as i32) as u32,
    )
}

fn main() {
    println!("Day 14");
    let file_path = "./input.txt";

    let contents = fs::read_to_string(file_path).expect("Coucou");

    let mut positions: Vec<(u32, u32)> = vec![];
    let mut speeds: Vec<(i32, i32)> = vec![];

    let re = Regex::new(r"(?m)^p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    for line in contents.lines() {
        for (_, [n1, n2, n3, n4]) in re.captures_iter(&line).map(|c| c.extract()) {
            positions.push((n1.parse::<u32>().unwrap(), n2.parse::<u32>().unwrap()));
            speeds.push((n3.parse::<i32>().unwrap(), n4.parse::<i32>().unwrap()));
        }
    }

    let mut current_positions = positions.clone();

    let (width, height) = (101, 103);

    let mut matrix: Vec<Vec<u32>> = vec![];

    for y in 0..height {
        matrix.push(vec![]);
        for _ in 0..width {
            matrix[y as usize].push(0)
        }
    }

    for _ in 0..100 {
        for i in 0..current_positions.len() {
            move_robot(i, &mut current_positions, &speeds, width, height)
        }
    }

    let (mut a, mut b, mut c, mut d) = (0, 0, 0, 0);

    for (x, y) in current_positions {
        if x < width / 2 && y < height / 2 {
            a += 1;
            continue;
        }
        if x < width / 2 && y > height / 2 {
            b += 1;
            continue;
        }
        if x > width / 2 && y < height / 2 {
            c += 1;
            continue;
        }
        if x > width / 2 && y > height / 2 {
            d += 1;
            continue;
        }
    }
    println!("{}", a * b * c * d);
}
