use std::fs;
use regex::Regex;
use dict::{ Dict, DictIface };

fn main() {
    println!("Day 5 Part 1");
    let file_path = "./example.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let re = Regex::new(r"(?m)^(\d+)\|(\d+)").unwrap();

    let mut results = vec![];
    let mut dict = Dict::<Vec<u64>>::new();
    for (_, [n1, n2]) in re.captures_iter(&contents).map(|c| c.extract()) {
        let a = n1.parse::<u64>() ? 0;
        let b = n2.parse::<u64>() ? 0;
        results.push((n1.parse::<u64>(), n2.parse::<u64>()));
        if (!dict.contains_key(a)) {
            dict.add(a.to_string(), vec![]);
        }
    }

    println!("Gros connard");
    //println!("{:?}", &ok[1]);
}
