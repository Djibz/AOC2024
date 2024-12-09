use std::fs;
use regex::Regex;

fn main() {
    println!("Day 3");
    let file_path = "./input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut part1_total = 0;

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for line in contents.lines() {
        for (_, [n1, n2]) in re.captures_iter(&line).map(|c| c.extract()) {
            let mul = n1.parse::<u64>().unwrap() * n2.parse::<u64>().unwrap();
            part1_total += mul;
        }
    }

    println!("{}", part1_total);

    let mut part2_total = 0;
    let mut activated = true;
    let re2 = Regex::new(r"mul\((\d+),(\d+)\)|(d)(o)n't\(\)|(d)(o)\(\)").unwrap();
    for line in contents.lines() {
        for (m, [n1, n2]) in re2.captures_iter(&line).map(|c| c.extract()) {
            if m == "don't()" {
                activated = false;
            } else if m == "do()" {
                activated = true;
            } else if activated {
                let mul = n1.parse::<u64>().unwrap() * n2.parse::<u64>().unwrap();
                part2_total += mul;
            }
        }
    }
    println!("{}", part2_total);
}
