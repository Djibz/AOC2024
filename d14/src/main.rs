use regex::Regex;
use std::fs;

use image::{ImageBuffer, Luma};

fn save_matrix_as_image(
    matrix: Vec<Vec<u32>>,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let height = matrix.len();
    let width = if height > 0 { matrix[0].len() } else { 0 };

    // Créer un buffer pour l'image
    let mut img = ImageBuffer::<Luma<u8>, _>::new(width as u32, height as u32);

    for (y, row) in matrix.iter().enumerate() {
        for (x, &value) in row.iter().enumerate() {
            // Si `value` est `true`, pixel blanc (255), sinon noir (0)
            let pixel_value = if value > 0 { 255 } else { 0 };
            img.put_pixel(x as u32, y as u32, Luma([pixel_value]));
        }
    }

    // Sauvegarder l'image au chemin donné
    img.save(output_path)?;

    Ok(())
}

fn move_robot(
    index: usize,
    positions: &mut Vec<(u32, u32)>,
    speeds: &Vec<(i32, i32)>,
    width: u32,
    height: u32,
) {
    let (x, y) = positions[index];
    let (sx, sy) = speeds[index];

    positions[index] = (
        ((width as i32 + x as i32 + sx) % width as i32) as u32,
        ((height as i32 + y as i32 + sy) % height as i32) as u32,
    )
}

fn main() {
    println!("Day 14");
    let file_path = "./input.txt";

    let contents = fs::read_to_string(file_path).expect("Coucou");

    let mut positions: Vec<(u32, u32)> = vec![];
    let mut speeds: Vec<(i32, i32)> = vec![];

    let re = Regex::new(r"(?m)^p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    for line in contents.lines() {
        for (_, [n1, n2, n3, n4]) in re.captures_iter(&line).map(|c| c.extract()) {
            positions.push((n1.parse::<u32>().unwrap(), n2.parse::<u32>().unwrap()));
            speeds.push((n3.parse::<i32>().unwrap(), n4.parse::<i32>().unwrap()));
        }
    }

    let mut current_positions = positions.clone();

    let (width, height) = (101, 103);

    let mut matrix: Vec<Vec<u32>> = vec![];

    for y in 0..height {
        matrix.push(vec![]);
        for _ in 0..width {
            matrix[y as usize].push(0)
        }
    }

    for second in 0..100 {
        let mut matrix: Vec<Vec<u32>> = vec![];
        for y in 0..height {
            matrix.push(vec![]);
            for _ in 0..width {
                matrix[y as usize].push(0)
            }
        }

        for i in 0..current_positions.len() {
            move_robot(i, &mut current_positions, &speeds, width, height);
            let (x, y) = current_positions[i];
            matrix[y as usize][x as usize] += 1;
        }

        let _ = save_matrix_as_image(matrix, format!("./images/{}.bmp", second).as_str());
    }

    let (mut a, mut b, mut c, mut d) = (0, 0, 0, 0);

    for (x, y) in current_positions.clone() {
        if x < width / 2 && y < height / 2 {
            a += 1;
            continue;
        }
        if x < width / 2 && y > height / 2 {
            b += 1;
            continue;
        }
        if x > width / 2 && y < height / 2 {
            c += 1;
            continue;
        }
        if x > width / 2 && y > height / 2 {
            d += 1;
            continue;
        }
    }
    println!("Part 1 : {}", a * b * c * d);

    for second in 100..10000 {
        let mut matrix: Vec<Vec<u32>> = vec![];
        for y in 0..height {
            matrix.push(vec![]);
            for _ in 0..width {
                matrix[y as usize].push(0)
            }
        }

        for i in 0..current_positions.len() {
            move_robot(i, &mut current_positions, &speeds, width, height);
            let (x, y) = current_positions[i];
            matrix[y as usize][x as usize] += 1;
        }

        let _ = save_matrix_as_image(matrix, format!("./images/{}.bmp", second).as_str());
    }
}
