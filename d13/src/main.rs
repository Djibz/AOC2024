use regex::Regex;
use std::fs;

fn main() {
    println!("Day 5");
    let file_path = "./example.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let re = Regex::new(
        r"(?m)Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();

    let mut total = 0;

    for (_, [n1, n2, n3, n4, n5, n6]) in re.captures_iter(&contents).map(|c| c.extract()) {
        println!("cool");
        let ax = n1.parse::<u128>().unwrap();
        let ay = n2.parse::<u128>().unwrap();
        let bx = n3.parse::<u128>().unwrap();
        let by = n4.parse::<u128>().unwrap();
        let mut tx = n5.parse::<u128>().unwrap();
        let mut ty = n6.parse::<u128>().unwrap();

        let a_mul = (10000000000000 / ax as u128).min(10000000000000 / ay as u128) / 2;
        let b_mul = (10000000000000 / bx as u128).min(10000000000000 / by as u128) / 2;

        println!("{}", (a_mul * ax) + ((b_mul) * bx));

        let max_iter = (tx / ax).max(tx / bx).max(ty / ay).max(ty / by) * 2;

        tx += 10000000000000;
        ty += 10000000000000;

        let mut min = 0;

        for a in 0..max_iter {
            for b in 0..max_iter {
                //println!("{}", ((a + a_mul) * ax) + ((b + b_mul) * bx));
                if ((a + a_mul) * ax) + ((b + b_mul) * bx) == tx
                    && ((a + a_mul) * ay) + ((b + b_mul) * by) == ty
                {
                    let tokens = 3 * a + b;

                    if min == 0 || tokens < min {
                        min = tokens;
                    }
                }
            }
        }

        total += min;
    }

    println!("Part 1 : {}", total);
}
