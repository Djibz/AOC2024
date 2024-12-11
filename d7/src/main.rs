use regex::Regex;
use std::fs;

fn concat(a: u128, b: u128) -> u128 {
    (a.to_string() + &b.to_string()).parse::<u128>().unwrap()
}

fn get_results(first_value: u128, from: usize, numbers: &Vec<u128>) -> Vec<u128> {
    if numbers.len() == from {
        return vec![first_value];
    }

    let mut r1 = get_results(first_value + numbers[from], from+1,&numbers);
    let mut r2 = get_results(first_value * numbers[from], from+1,&numbers);
    let mut r3 = get_results(concat(first_value, numbers[from]), from+1,&numbers);
    
    r1.append(&mut r2);
    r1.append(&mut r3);

    return r1;
}

fn main() {
    println!("Day 7");
    let file_path = "./input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let re = Regex::new(r"(?m)^(\d+):\s+(.+)").unwrap();

    let mut total: u128 = 0;

    for (_, [m, l]) in re.captures_iter(&contents).map(|c| c.extract()) {
        let wanted = m.parse::<u128>().unwrap();
        let numbers: Vec<u128> = l
            .split_whitespace()
            .map(|x| x.parse::<u128>().unwrap())
            .collect();


        let results = get_results(numbers[0], 1, &numbers);

        if results.contains(&wanted) {
            total += wanted;
        }
    }
    println!("Part 1-2 : {}", total);
}
