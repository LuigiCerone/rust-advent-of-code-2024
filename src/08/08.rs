use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_matrix(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let lines = io::BufReader::new(file).lines();

    let mut matrix = Vec::new();

    for line in lines {
        let line = line?;
        let values: Vec<char> = line.trim().chars().collect();
        matrix.push(values);
    }

    Ok(matrix)
}

fn compute_antennas(matrix: &Vec<Vec<char>>) -> HashMap<char, Vec<(i32, i32)>> {
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (r, row) in matrix.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell != '.' {
                antennas
                    .entry(cell)
                    .or_insert_with(Vec::new)
                    .push((r as i32, c as i32));
            }
        }
    }

    antennas
}

fn run_1(antennas: &HashMap<char, Vec<(i32, i32)>>, matrix: &Vec<Vec<char>>) -> usize {
    let mut antinodes_set: HashSet<(i32, i32)> = HashSet::new();

    for positions in antennas.values() {
        let combinations = generate_combinations(positions);

        for (a, b) in combinations {
            let (x1, y1) = a;
            let (x2, y2) = b;

            let vector_x = x2 - x1;
            let vector_y = y2 - y1;

            let antinodes = vec![
                (x1 - vector_x, y1 - vector_y), // Negative direction
                (x2 + vector_x, y2 + vector_y), // Positive direction
            ];

            for (x, y) in antinodes {
                if is_in_matrix(x, y, &matrix) {
                    antinodes_set.insert((x, y));
                }
            }
        }
    }

    antinodes_set.len()
}

fn generate_combinations(positions: &[(i32, i32)]) -> Vec<((i32, i32), (i32, i32))> {
    let mut combinations = Vec::new();
    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            combinations.push((positions[i], positions[j]));
        }
    }
    combinations
}

fn is_in_matrix(x: i32, y: i32, matrix: &[Vec<char>]) -> bool {
    x >= 0 && y >= 0 && (x as usize) < matrix.len() && (y as usize) < matrix[0].len()
}

fn run_2(antennas: &HashMap<char, Vec<(i32, i32)>>, matrix: &Vec<Vec<char>>) -> usize {
    let mut antinodes_set: HashSet<(i32, i32)> = HashSet::new();

    for positions in antennas.values() {
        let combinations = generate_combinations(positions);

        for (a, b) in combinations {
            let (x1, y1) = a;
            let (x2, y2) = b;

            let vector_x = x2 - x1;
            let vector_y = y2 - y1;

            let mut i = 0;
            loop {
                // Positive direction
                let x_pos = x1 + i * vector_x;
                let y_pos = y1 + i * vector_y;

                // Negative direction
                let x_neg = x2 - i * vector_x;
                let y_neg = y2 - i * vector_y;

                let mut should_break = true;

                // Check and add positions within bounds
                if is_in_matrix(x_pos, y_pos, &matrix) {
                    antinodes_set.insert((x_pos, y_pos));
                    should_break = false;
                }
                if is_in_matrix(x_neg, y_neg, &matrix) {
                    antinodes_set.insert((x_neg, y_neg));
                    should_break = false;
                }

                // Stop the loop if both directions are out of bounds
                if should_break {
                    break;
                }

                i += 1;
            }
        }
    }

    antinodes_set.len()
}

fn main() -> io::Result<()> {
    let file_path = "src/08/input.txt";

    let matrix = read_matrix(file_path).unwrap();

    let antenna_positions = compute_antennas(&matrix);

    let result_first_quiz = run_1(&antenna_positions, &matrix);
    println!("{}", result_first_quiz);

    let result_second_quiz = run_2(&antenna_positions, &matrix);
    println!("{}", result_second_quiz);

    Ok(())
}
