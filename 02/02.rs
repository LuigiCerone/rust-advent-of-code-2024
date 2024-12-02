use std::fs::File;
use std::io::{self, BufRead};

fn read_file(file_path: &str) -> io::Result<Vec<Vec<i32>>> {
    let mut matrix = Vec::new();

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let values: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|v| v.parse::<i32>().unwrap()) // Parse each value as i32
            .collect();
        matrix.push(values);
    }

    Ok(matrix)
}

fn process_row(row: &[i32]) -> bool {
    let mut difference = row[0] - row[1];

    for index in 1..row.len() {
        let new_difference = row[index - 1] - row[index];

        if new_difference == 0
            || new_difference.abs() > 3
            || (new_difference > 0) != (difference > 0)
        {
            return false;
        }

        difference = new_difference;
    }

    true
}

fn run_1(matrix: &[Vec<i32>]) -> i32 {
    matrix
        .iter()
        .map(|row| if process_row(row) { 1 } else { 0 })
        .sum()
}

fn process_row_2(row: &[i32]) -> bool {
    for i in 0..row.len() {
        let mut new_row = Vec::with_capacity(row.len() - 1);

        new_row.extend_from_slice(&row[0..i]);
        new_row.extend_from_slice(&row[i + 1..]);

        if process_row(&new_row) {
            return true;
        }
    }

    false
}

fn run_2(matrix: &[Vec<i32>]) -> i32 {
    matrix
        .iter()
        .map(|row| if process_row_2(row) { 1 } else { 0 })
        .sum()
}

fn main() -> io::Result<()> {
    let file_path = "input.txt";

    let matrix = read_file(file_path)?;

    let result_first_quiz = run_1(&matrix);

    println!("{}", result_first_quiz);

    let result_second_quiz = run_2(&matrix);

    println!("{}", result_second_quiz);

    Ok(())
}
