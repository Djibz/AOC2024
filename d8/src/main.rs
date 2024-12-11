use std::collections::HashMap;
use std::fs;

fn main() {
    println!("Day 8");
    let file_path = "./input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut height: usize = 0;
    let mut width: usize = 0;

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            width = x + 1;
            if c == '.' {
                continue;
            }

            if !antennas.contains_key(&c) {
                antennas.insert(c, vec![]);
            }

            antennas.get_mut(&c).unwrap().push((x, y));

            width = x + 1;
        }
        height = y + 1;
    }
    println!("Size {}x{}", width, height);

    let mut antinodes: Vec<(i32, i32)> = vec![];

    for (_, positions) in &antennas {
        let size = positions.len();
        for i in 0..size {
            for j in (i + 1)..size {
                let (xa, ya) = positions[i];
                let (xb, yb) = positions[j];

                let x_dist: i32 = (xa as i32) - (xb as i32);
                let y_dist: i32 = (ya as i32) - (yb as i32);

                antinodes.push(((xa as i32) + x_dist, (ya as i32) + y_dist));
                antinodes.push(((xb as i32) - x_dist, (yb as i32) - y_dist));
            }
        }
    }

    // Filter antinodes
    antinodes.sort();
    antinodes.dedup();

    antinodes = antinodes
        .into_iter()
        .filter(|(x, y)| *x >= 0 && *x < (width as i32) && *y >= 0 && *y < (height as i32))
        .collect();

    println!("Part 1 : {}", antinodes.len());

    antinodes = vec![];

    for (_, positions) in &antennas {
        let size = positions.len();
        for i in 0..size {
            for j in (i + 1)..size {
                let (xa, ya) = positions[i];
                let (xb, yb) = positions[j];

                let x_dist: i32 = (xa as i32) - (xb as i32);
                let y_dist: i32 = (ya as i32) - (yb as i32);

                for r in 0..=50 {
                    antinodes.push(((xa as i32) + (r * x_dist), (ya as i32) + (r * y_dist)));
                    antinodes.push(((xb as i32) - (r * x_dist), (yb as i32) - (r * y_dist)));
                }
            }
        }
    }

    // Filter antinodes
    antinodes.sort();
    antinodes.dedup();

    antinodes = antinodes
        .into_iter()
        .filter(|(x, y)| *x >= 0 && *x < (width as i32) && *y >= 0 && *y < (height as i32))
        .collect();

    println!("Part 2 : {}", antinodes.len());
}
