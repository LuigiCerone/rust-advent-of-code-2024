use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const ALLOWED_DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn read_matrix(filename: &str) -> io::Result<Vec<Vec<i32>>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let lines = io::BufReader::new(file).lines();

    let mut matrix: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        let line = line?;
        let values: Vec<i32> = line
            .chars() // Iterate over each character
            .filter(|c| c.is_digit(10)) // Ensure it's a digit
            .map(|c| c.to_digit(10).unwrap() as i32) // Convert char to i32
            .collect();
        matrix.push(values);
    }

    Ok(matrix)
}


fn is_in_matrix(point: (isize, isize), matrix: &Vec<Vec<i32>>) -> bool {
    let (row, col) = point;
    row >= 0
        && col >= 0
        && (row as usize) < matrix.len()
        && (col as usize) < matrix[0].len()
}



fn find_starting_points(matrix: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut starts = Vec::new();

    for (row, row_data) in matrix.iter().enumerate() {
        for (col, &value) in row_data.iter().enumerate() {
            if value == 0 {
                starts.push((row, col));
            }
        }
    }

    starts
}


fn dfs_explore_map(
    current_point: (usize, usize),
    matrix: &Vec<Vec<i32>>,
) -> Vec<(isize, isize)> {
    let mut result = Vec::new();
    let (curr_x, curr_y) = current_point;

    if matrix[curr_x][curr_y] == 9 {
        return vec![(curr_x as isize, curr_y as isize)];
    }

    for &(dx, dy) in &ALLOWED_DIRECTIONS {
        let next_x = curr_x as isize + dx;
        let next_y = curr_y as isize + dy;

        // Check bounds and access the matrix only if valid
        if is_in_matrix((next_x, next_y), matrix) {
            let next_x_usize = next_x as usize;
            let next_y_usize = next_y as usize;

            if matrix[next_x_usize][next_y_usize] == matrix[curr_x][curr_y] + 1 {
                result.extend(dfs_explore_map((next_x_usize, next_y_usize), matrix));
            }
        }
    }

    result
}


fn run(matrix: &Vec<Vec<i32>>, part_one: bool) -> usize {
    let mut result = 0;
    let starts = find_starting_points(&matrix);

    for &(start_x, start_y) in &starts {
        let score = dfs_explore_map((start_x, start_y), &matrix);

        if part_one {
            let unique_scores: std::collections::HashSet<(isize, isize)> = score.into_iter().collect();
            result += unique_scores.len();
        } else {
            result += score.len();
        }
    }

    result
}

fn main() -> io::Result<()> {
    let file_path = "src/10/input.txt";

    let matrix = read_matrix(file_path).unwrap();
    // println!("{:?}", matrix);

    let result_first_quiz = run(&matrix, true);
    println!("{}", result_first_quiz);

    
    let result_second_quiz = run(&matrix, false);
    println!("{}", result_second_quiz);
    

    Ok(())
}