use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn read_file(file_path: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let columns: Vec<&str> = line.split_whitespace().collect();

        if columns.len() == 2 {
            column1.push(columns[0].parse().unwrap());
            column2.push(columns[1].parse().unwrap());
        }
    }

    Ok((column1, column2))
}

fn run_1(list_a: &mut Vec<i32>, list_b: &mut Vec<i32>) -> i32 {
    list_a.sort();
    list_b.sort();

    let mut result = 0;

    for index in 0..list_a.len() {
        // Calculate the absolute distance between the corresponding elements
        let distance = (list_a[index] - list_b[index]).abs();
        result += distance;
    }

    result
}

fn run_2(list_a: Vec<i32>, list_b: Vec<i32>) -> i32 {
    let mut counter: HashMap<i32, i32> = HashMap::new();
    let mut result = 0;

    // Count occurrences of each element in list_b
    for &n in &list_b {
        *counter.entry(n).or_insert(0) += 1;
    }

    // Calculate the result by summing (n * count) for each element in list_a
    for &n in &list_a {
        result += n * counter.get(&n).unwrap_or(&0);
    }

    result
}


fn main() -> io::Result<()> {
    let file_path = "input.txt";

    let (mut column1, mut column2) = read_file(file_path)?;

    let result_first_quiz = run_1(&mut column1, &mut column2);

    println!("{}", result_first_quiz);

    let result_second_quiz = run_2(column1, column2);

    println!("{}", result_second_quiz);

    Ok(())
}
