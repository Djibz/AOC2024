use std::fs;

fn get_path(
    position: (usize, usize),
    previous: (usize, usize),
    paths: &Vec<(usize, usize)>,
    end: &(usize, usize),
) -> Vec<(usize, usize)> {
    if position == *end {
        return vec![*end];
    }
}

fn main() {
    println!("Day 20");
    let file_path = "./example.txt";

    let contents = fs::read_to_string(file_path).expect("Nul");

    let mut h: usize = 0;
    let mut w: usize = 0;

    let mut paths: Vec<(usize, usize)> = vec![];
    let mut walls: Vec<(usize, usize)> = vec![];

    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

    for (y, line) in contents.lines().enumerate() {
        h = y;
        for (x, c) in line.chars().enumerate() {
            w = x;
            match c {
                '.' => paths.push((x, y)),
                'S' => start = (x, y),
                'E' => end = (x, y),
                _ => walls.push((x, y)),
            }
        }
    }

    println!("Start : {:?}", start);
}
