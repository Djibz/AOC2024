use std::fs;

fn get_reachable(
    y: usize,
    x: usize,
    matrix: &Vec<Vec<u32>>,
    w: usize,
    h: usize,
) -> Vec<(usize, usize)> {
    let n = matrix[x][y];
    if n == 9 {
        println!("{:?}", (x, y));
        return vec![(x, y)];
    }

    let mut reachable: Vec<(usize, usize)> = vec![];

    if x != 0 && matrix[x - 1][y] == n + 1 {
        let mut a = get_reachable(x - 1, y, matrix, w, h);
        reachable.append(&mut a);
    }
    if y != 0 && matrix[x][y - 1] == n + 1 {
        let mut a = get_reachable(x, y - 1, matrix, w, h);
        reachable.append(&mut a);
    }
    if x + 1 < w && matrix[x + 1][y] == n + 1 {
        reachable.append(&mut get_reachable(x + 1, y, matrix, w, h));
    }
    if y + 1 < h && matrix[x][y + 1] == n + 1 {
        reachable.append(&mut get_reachable(x, y + 1, matrix, w, h));
    }

    return reachable.clone();
}

fn main() {
    println!("Day 10");
    let file_path = "./example.txt";

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
    println!("{:?}", starts);

    let mut total = 0;

    for (x, y) in starts {
        let mut ends = get_reachable(x, y, &heights, w, h);
        ends.sort();
        ends.dedup();

        println!("{:?}", ends);

        total += ends.len();
    }

    println!("Part 1 : {}", total)
}
