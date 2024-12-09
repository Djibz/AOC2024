use std::fs;

fn is_ok(list: Vec<u32>) -> bool {
    let asc = list[0] < list[1];

    for i in 0..list.len() {
        if i == 0 {
            continue;
        }

        if asc && list[i-1] > list[i] {
            return false;
        }
        if !asc && list[i-1] < list[i] {
            return false;
        }

        let diff = list[i-1].abs_diff(list[i]);

        if diff < 1 || diff > 3 {
            return false
        }
    }
    return true;
}

fn main() {
    println!("Day 2");
    let file_path = "./input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut part1_total = 0;
    let mut part2_total = 0;
    for line in contents.lines() {
        let nbs: Vec<u32> = line.split_whitespace()
            .map(|n| n.parse::<u32>()
            .unwrap())
            .collect();

        if is_ok(nbs.clone()) {
            part1_total += 1;
            part2_total += 1;
        } else {
            for i in 0..nbs.len() {
                let v2 = [&nbs[..i], &nbs[i+1..]].concat();

                if is_ok(v2.clone()) {
                    part2_total += 1;
                    break;
                }
            }
        }
    }

    print!("Part 1 : {}\n", part1_total);
    print!("Part 2 : {}\n", part2_total);
}
