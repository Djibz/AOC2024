use dict::{Dict, DictIface};
use regex::Regex;
use std::{cmp::Ordering, fs};

fn get_middle_value(list: Vec<u64>) -> u64 {
    return list[list.len() / 2];
}

fn compare_pages(&a: &u64, &b: &u64, dict: &Dict<Vec<u64>>) -> Ordering {
    let empty = Vec::<u64>::new();

    let befores = match dict.get(&a.to_string()) {
        Some(v) => v,
        None => &empty,
    };

    if befores.iter().any(|j| j == &b) {
        return Ordering::Greater;
    }

    let befores2 = match dict.get(&b.to_string()) {
        Some(v) => v,
        None => &empty,
    };

    if befores2.iter().any(|j| j == &a) {
        return Ordering::Less;
    }

    return Ordering::Equal;
}

fn main() {
    println!("Day 5");
    let file_path = "./input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

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

    let mut total = 0;
    let mut total2 = 0;

    let re2 = Regex::new(r"(?m)^\d+(?:,\d+)+").unwrap();
    for (m, []) in re2.captures_iter(&contents).map(|c| c.extract()) {
        let mut nbs: Vec<u64> = m.split(',').map(|n| n.parse::<u64>().unwrap()).collect();

        // println!("{:?}", nbs);

        let empty = Vec::<u64>::new();
        let mut valid = true;
        for i in 0..nbs.len() {
            let befores = match dict.get(&nbs[i].to_string()) {
                Some(v) => v,
                None => &empty,
            };

            for n in befores {
                if nbs[(i + 1)..].iter().any(|j| j == n) {
                    valid = false;
                }
            }
        }

        if valid {
            total += get_middle_value(nbs)
        } else {
            nbs.sort_by(|v, w| compare_pages(v, w, &dict));
            total2 += get_middle_value(nbs)
        }
    }

    println!("Part 1 : {}", total);
    println!("Part 2 : {}", total2);
}
