use std::fs;

fn get_reachable(
    x: usize,
    y: usize,
    matrix: &Vec<Vec<u32>>,
    w: usize,
    h: usize,
) -> Vec<(usize, usize)> {
    let n = matrix[y][x];
    if n == 9 {
        return vec![(x, y)];
    }

    let mut reachable: Vec<(usize, usize)> = vec![];

    if x != 0 && matrix[y][x - 1] == n + 1 {
        reachable.extend(get_reachable(x - 1, y, &matrix, w, h));
    }
    if y != 0 && matrix[y - 1][x] == n + 1 {
        reachable.extend(get_reachable(x, y - 1, &matrix, w, h));
    }
    if x + 1 < w && matrix[y][x + 1] == n + 1 {
        reachable.extend(get_reachable(x + 1, y, &matrix, w, h));
    }
    if y + 1 < h && matrix[y + 1][x] == n + 1 {
        reachable.extend(get_reachable(x, y + 1, &matrix, w, h));
    }

    return reachable.clone();
}

fn main() {
    println!("Day 10");
    let file_path = "./input.txt";

    let contents = fs::read_to_string(file_path).expect("Nul");

    let mut h: usize = 0;
    let mut w: usize = 0;

    let mut heights: Vec<Vec<u32>> = vec![];
    let mut starts: Vec<(usize, usize)> = vec![];
    for (y, line) in contents.lines().enumerate() {
        heights.push(vec![]);
        for (x, c) in line.chars().enumerate() {
            let n = c.to_digit(10).unwrap();
            heights[y].push(n);
            w = x + 1;

            if n == 0 {
                starts.push((x, y));
            }
        }
        h = y + 1;
    }

    println!("{}x{}", h, w);

    let mut total = 0;
    let mut total2 = 0;

    for (x, y) in starts {
        let mut ends = get_reachable(x, y, &heights, w, h);
        total2 += ends.len();

        ends.sort();
        ends.dedup();

        total += ends.len();
    }

    println!("Part 1 : {}", total);
    println!("Part 1 : {}", total2)
}
