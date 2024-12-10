use regex::Regex;
use std::fs;

fn get_results(&mut numbers: &mut Vec<u128>) -> Vec<u128> {
    if numbers.len() == 1 {
        return numbers;
    }

    let mut add: Vec<u128> = vec![numbers[0] + numbers[1]];
    let mut mul: Vec<u128> = vec![numbers[0] * numbers[1]];
    add.append(&mut numbers[1..].to_vec());
    mul.append(&mut numbers[1..].to_vec());

    let mut r1 = get_results(&mut add);
    let mut r2 = get_results(&mut mul);

    r1.append(&mut r2);

    return r1;
}

fn main() {
    println!("Day 1");
    let file_path = "./example.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let re = Regex::new(r"(?m)^(\d+):\s+(.+)").unwrap();

    for (_, [m, l]) in re.captures_iter(&contents).map(|c| c.extract()) {
        let wanted = m.parse::<u128>().unwrap();
        let numbers: Vec<u128> = l
            .split_whitespace()
            .map(|x| x.parse::<u128>().unwrap())
            .collect();

        println!("{}", wanted);
        println!("{:?}", numbers);

        let result = numbers[0];

        for n in 1..numbers.len() {}
    }
}
