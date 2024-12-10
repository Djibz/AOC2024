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
        let b = n1.parse::<u64>().unwrap();
        let a = n2.parse::<u64>().unwrap();
    
        results.push((n1.parse::<u64>(), n2.parse::<u64>()));

        if !dict.contains_key(&a.to_string()) {
            dict.add(a.to_string(), vec![]);
        }

        let mut ok: Vec<u64> = (*dict.get(&a.to_string()).unwrap()).clone();
        ok.push(b);
        dict.remove_key(&a.to_string());
        dict.add(a.to_string(), ok);
    }

    println!("Gros connard");

    let mut total = 0;

    let re2 = Regex::new(r"(?m)^\d+(?:,\d+)+").unwrap();
    for (m, []) in re2.captures_iter(&contents).map(|c| c.extract()) {
        let nbs: Vec<u64> = m.split(',')
            .map(|n| n.parse::<u64>()
            .unwrap())
            .collect();

        println!("{:?}", nbs);

        let empty = Vec::<u64>::new();
        let mut valid = false;
        for i in 0..nbs.len() {
            let befores = match dict.get(&nbs[i].to_string()) {
                Some(v) => v,
                None => &empty,
            };
            for n in befores {
                if nbs[(i+1)..].iter().any(|j| j==n) {
                    valid = false;
                }
            }
        }

        if valid {
            total += 1
        }
    }

    println!("{}", total);
}
