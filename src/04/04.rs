use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn count_occurrences(string: &str, search_term: &str) -> usize {
    string.matches(search_term).count()
}

fn count_1(string: &str) -> usize {
    count_occurrences(string, "XMAS") + count_occurrences(string, "SAMX")
}

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



fn run_1(matrix: &Vec<Vec<char>>) -> usize {
    let n_rows = matrix.len();
    let n_cols = matrix[0].len();
    let mut result = 0;

    // Horizontally
    for row_as_list in matrix {
        let row_str: String = row_as_list.iter().collect();
        result += count_1(&row_str);
    }

    // Vertically
    for i_col in 0..n_cols {
        let mut s = String::new();
        for i_row in 0..n_rows {
            s.push(matrix[i_row][i_col]);
        }
        result += count_1(&s);
    }

    // Diagonal from top-left to bottom-right
    for start in 0..(n_rows + n_cols - 1) {
        let mut diag = Vec::new();
        for row in 0..n_rows {
            let col = start as isize - row as isize; // Cast to isize for arithmetic
            if col >= 0 && col < n_cols as isize { // Check for valid column
                diag.push(matrix[row][col as usize]); // Cast back to usize for indexing
            }
        }
        let diag_str: String = diag.iter().collect();
        result += count_1(&diag_str);
    }

    // Diagonal from top-right to bottom-left
    for start in 0..(n_rows + n_cols - 1) {
        let mut diag = Vec::new();
        for row in 0..n_rows {
            let col = start as isize - (n_rows as isize - 1 - row as isize); // Cast to isize for arithmetic
            if col >= 0 && col < n_cols as isize { // Check for valid column
                diag.push(matrix[row][col as usize]); // Cast back to usize for indexing
            }
        }
        let diag_str: String = diag.iter().collect();
        result += count_1(&diag_str);
    }

    result
}


fn run_2(matrix: &Vec<Vec<char>>) -> usize {
    let n_rows = matrix.len();
    let n_cols = matrix[0].len();

    let mut result = 0;
    let candidates: HashSet<String> = vec![
        "MASSAM".to_string(),
        "MASMAS".to_string(),
        "SAMSAM".to_string(),
        "SAMMAS".to_string(),
    ]
    .into_iter()
    .collect();

    for i in 1..n_rows - 1 {
        for j in 1..n_cols - 1 {
            let x_cross: String = vec![
                matrix[i - 1][j - 1], // top-left diagonal
                matrix[i][j],         // center
                matrix[i + 1][j + 1], // bottom-right diagonal
                matrix[i - 1][j + 1], // top-right diagonal
                matrix[i][j],         // center
                matrix[i + 1][j - 1], // bottom-left diagonal
            ]
            .iter()
            .collect(); // Collecting the characters into a single string

            if candidates.contains(&x_cross) {
                result += 1;
            }
        }
    }

    result
}


fn main() -> io::Result<()> {
    let file_path = "src/04/input.txt";

    let input_string = read_matrix(file_path)?;

    let result_first_quiz = run_1(&input_string);

    println!("{}", result_first_quiz);

    let result_second_quiz = run_2(&input_string);

    println!("{}", result_second_quiz);

    Ok(())
}

