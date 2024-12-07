use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

fn read_file(filename: &str) -> io::Result<(Vec<i64>, Vec<Vec<i64>>)> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut results = Vec::new();
    let mut sequences = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(':').collect();

        let result = parts[0].trim().parse().unwrap();
        results.push(result);

        let values = parts[1]
            .trim()
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        sequences.push(values);
    }

    Ok((results, sequences))
}

fn dfs_1(
    curr_index: usize,
    curr_result: i64,
    dp: &mut HashMap<usize, HashSet<i64>>,
    seq: &Vec<i64>,
) {
    dp.entry(curr_index)
        .or_insert(HashSet::new())
        .insert(curr_result);

    if curr_index == seq.len() {
        return;
    }

    // Recursive calls
    dfs_1(curr_index + 1, curr_result * seq[curr_index], dp, seq);
    dfs_1(curr_index + 1, curr_result + seq[curr_index], dp, seq);
}

fn run_1(results: &Vec<i64>, sequences: &Vec<Vec<i64>>) -> i64 {
    let mut result = 0;

    for (i, &target) in results.iter().enumerate() {
        let seq = &sequences[i];
        let mut dp: HashMap<usize, HashSet<i64>> = HashMap::new();

        dfs_1(0, 0, &mut dp, seq);

        if let Some(final_set) = dp.get(&seq.len()) {
            if final_set.contains(&target) {
                result += target;
            }
        }
    }

    result
}

fn dfs_2(
    curr_index: usize,
    curr_result: i64,
    dp: &mut HashMap<usize, HashSet<i64>>,
    seq: &Vec<i64>,
) {
    dp.entry(curr_index)
        .or_insert(HashSet::new())
        .insert(curr_result);

    if curr_index == seq.len() {
        return;
    }

    // Recursive calls
    dfs_2(curr_index + 1, curr_result * seq[curr_index], dp, seq);
    dfs_2(curr_index + 1, curr_result + seq[curr_index], dp, seq);

    let concatenated = format!("{}{}", curr_result, seq[curr_index])
        .parse::<i64>()
        .unwrap_or(0); // Fallback to 0 if parse fails

    dfs_2(curr_index + 1, concatenated, dp, seq);
}

fn run_2(results: &Vec<i64>, sequences: &Vec<Vec<i64>>) -> i64 {
    let mut result = 0;

    for (i, &target) in results.iter().enumerate() {
        let seq = &sequences[i];
        let mut dp: HashMap<usize, HashSet<i64>> = HashMap::new();

        dfs_2(0, 0, &mut dp, seq);

        if let Some(final_set) = dp.get(&seq.len()) {
            if final_set.contains(&target) {
                result += target;
            }
        }
    }

    result
}

fn main() -> io::Result<()> {
    let file_path = "src/07/input.txt";

    let (results, sequences) = read_file(file_path).unwrap();

    let result_first_quiz = run_1(&results, &sequences);
    println!("{}", result_first_quiz);

    let result_second_quiz = run_2(&results, &sequences);

    println!("{}", result_second_quiz);

    Ok(())
}
