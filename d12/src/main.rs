use std::fs;

fn get_infos(
    x: usize,
    y: usize,
    w: &usize,
    h: &usize,
    map: &mut Vec<Vec<char>>,
    went: &mut Vec<Vec<bool>>,
) -> (u32, u32, Vec<(usize, usize)>) {
    let c = map[y][x];
    went[y][x] = true;

    let mut area = 1;
    let mut perimeter = 0;
    let mut sides: Vec<(usize, usize)> = vec![];

    let mut a_side = false;

    if x > 0 && map[y][x - 1] == c {
        if !went[y][x - 1] {
            let (a, b, s) = get_infos(x - 1, y, w, h, map, went);
            area += a;
            perimeter += b;
            sides.extend(s);
        }
    } else {
        perimeter += 1;
        a_side = true;
    }

    if y > 0 && map[y - 1][x] == c {
        if !went[y - 1][x] {
            let (a, b, s) = get_infos(x, y - 1, w, h, map, went);
            area += a;
            perimeter += b;
            sides.extend(s);
        }
    } else {
        perimeter += 1;
        a_side = true;
    }

    if x + 1 < *w && map[y][x + 1] == c {
        if !went[y][x + 1] {
            let (a, b, s) = get_infos(x + 1, y, w, h, map, went);
            area += a;
            perimeter += b;
            sides.extend(s);
        }
    } else {
        perimeter += 1;
        a_side = true;
    }

    if y + 1 < *h && map[y + 1][x] == c {
        if !went[y + 1][x] {
            let (a, b, s) = get_infos(x, y + 1, w, h, map, went);
            area += a;
            perimeter += b;
            sides.extend(s);
        }
    } else {
        perimeter += 1;
        a_side = true;
    }

    if a_side {
        sides.push((x, y));
    }

    return (area, perimeter, sides.clone());
}

fn main() {
    println!("Day 12");
    let file_path = "./example.txt";

    let contents = fs::read_to_string(file_path).expect("Nul");

    let mut h: usize = 0;
    let mut w: usize = 0;

    let mut zones: Vec<Vec<char>> = vec![];
    let mut went: Vec<Vec<bool>> = vec![];
    for (y, line) in contents.lines().enumerate() {
        h = y + 1;
        zones.push(vec![]);
        went.push(vec![]);
        for (x, c) in line.chars().enumerate() {
            zones[y].push(c);
            went[y].push(false);
            w = x + 1;
        }
    }

    let mut total = 0;
    for x in 0..w {
        for y in 0..h {
            if !went[y][x] {
                let (a, p, sides) = get_infos(x, y, &w, &h, &mut zones, &mut went);
                println!("{:?}", sides);
                total += a * p;
            }
        }
    }

    println!("{}", total);
}
