use regex::Regex;
use std::{cmp::Ordering, fs};

fn main() {
    println!("Day 5");
    let file_path = "./example.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let re = Regex::new(
        r"(?m)Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    for (_, [n1, n2, n3, n4, n5, n6]) in re.captures_iter(&contents).map(|c| c.extract()) {
        let ax = n1.parse::<u32>().unwrap();
        let ay = n2.parse::<u32>().unwrap();
        let bx = n3.parse::<u32>().unwrap();
        let by = n4.parse::<u32>().unwrap();
        let tx = n5.parse::<u32>().unwrap();
        let ty = n6.parse::<u32>().unwrap();
    }
}
