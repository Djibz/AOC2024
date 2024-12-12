use std::fs;

fn main() {
    println!("Day 9");
    let file_path = "./input.txt";

    let mut contents =
        fs::read_to_string(file_path).expect("Should have been able to read the file");
    contents.pop();

    let mut count_files: u32 = 0;
    let mut count_empty: u32 = 0;

    let mut files = vec![];
    let mut empties: Vec<bool> = vec![];

    for (id, c) in contents.chars().enumerate() {
        let n = c.to_digit(10).unwrap();
        if id & 1 == 0 {
            count_files += n as u32;
            for _ in 0..n {
                files.push((id / 2) as u32);
                empties.push(false);
            }
        } else {
            count_empty += n as u32;
            for _ in 0..n {
                empties.push(true);
            }
        }
    }

    println!("{} {}", count_files, count_empty);

    let mut total: u128 = 0;
    let mut index = 0;
    let mut back_index = files.len() - 1;

    for i in 0..count_files {
        if empties[i as usize] {
            total += (i * files[back_index as usize]) as u128;
            back_index -= 1;
        } else {
            total += (i * files[index as usize]) as u128;
            index += 1;
        }
    }

    println!("Part 1 : {}", total);
}
