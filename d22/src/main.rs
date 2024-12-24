use std::{fs, u128};

fn mix(nb: u128, secret: u128) -> u128 {
    nb ^ secret
}

fn prune(nb: u128) -> u128 {
    nb % 16777216
}

fn get_next_key(key: u128) -> u128 {
    let k1 = prune(mix(key * 64, key));
    let k2 = prune(mix(k1 / 32, k1));
    return prune(mix(k2 * 2048, k2));
}

fn nth_key(start: u128, n: u128) -> u128 {
    if n == 0 {
        return start;
    }

    return nth_key(get_next_key(start), n - 1);
}

fn main() {
    println!("Day 22");
    let file_path = "./input.txt";

    let contents = fs::read_to_string(file_path).expect("Prout");

    let mut part1_total = 0;
    for line in contents.lines() {
        let key = line.parse::<u128>().unwrap();
        part1_total += nth_key(key, 2000)
    }
    println!("Part 1 : {}", part1_total)
}
