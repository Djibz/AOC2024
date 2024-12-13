use memoize::memoize;
use std::fs;

#[memoize]
fn count_final(number: u128, limit: u128) -> u128 {
    if limit == 0 {
        return 1;
    }

    if number == 0 {
        return count_final(1, limit - 1);
    }

    let s_number = number.to_string();

    if s_number.len() & 1 == 0 {
        let (a, b) = s_number.split_at(s_number.len() / 2);
        return count_final(a.parse::<u128>().unwrap(), limit - 1)
            + count_final(b.parse::<u128>().unwrap(), limit - 1);
    }

    return count_final(number * 2024, limit - 1);
}

fn main() {
    println!("Day 11");
    let file_path = "./input.txt";

    let contents = fs::read_to_string(file_path).expect("Prout");

    let total: u128 = contents
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| count_final(n.parse::<u128>().unwrap(), 75))
        .reduce(|a, b| a + b)
        .unwrap();

    println!("{:?}", total);
}
