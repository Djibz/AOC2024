use std::fs;
use regex::Regex;

fn main() {
    println!("Day 1");
    let file_path = "./input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let re = Regex::new(r"(?m)^(\d+)\s+(\d+)").unwrap();

    let mut first = vec![];
    let mut second = vec![];

    for (_, [n1, n2]) in re.captures_iter(&contents).map(|c| c.extract()) {
        let a = n1.parse::<u64>().unwrap();
        let b = n2.parse::<u64>().unwrap();

        first.push(a);
        second.push(b);
    }

    first.sort();
    second.sort();

    let mut total = 0;

    for i in 0..first.len() {
        total += first[i].abs_diff(second[i]);
    }

    print!("Part 1 : {}\n", total);

    let size = first.len();

    total = 0;
    for i in 0..size {
        let mut count = 0;
        for j in 0..size {
            if first[i] == second[j] {
                count += 1
            }
        }
        total += first[i] * count;
    }

    print!("Part 2 : {}\n", total);
}
