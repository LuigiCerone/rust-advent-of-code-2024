use std::fs::File;
use std::io::{self, BufRead};

fn read_file(filename: &str) -> io::Result<(Vec<(i32, i32)>, Vec<Vec<i32>>)> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut seen_newline = true;
    let mut rules = Vec::new();
    let mut sequences = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            seen_newline = false;
            continue;
        }

        if seen_newline {
            let parts: Vec<i32> = line
                .trim()
                .split('|')
                .filter_map(|x| x.trim().parse().ok())
                .collect();
            rules.push((parts[0], parts[1]));
        } else {
            let parsed: Vec<i32> = line
                .trim()
                .split(',')
                .filter_map(|x| x.trim().parse().ok())
                .collect();
            sequences.push(parsed);
        }
    }

    Ok((rules, sequences))
}

fn run_1(rules: &Vec<(i32, i32)>, sequences: &Vec<Vec<i32>>) -> (i32, Vec<Vec<i32>>) {
    let mut result = 0;
    let mut invalid_sequences = Vec::new();

    for seq in sequences {
        if is_valid_seq(rules, seq) {
            let mid_index = (seq.len() - 1) / 2;
            result += seq[mid_index];
        } else {
            invalid_sequences.push(seq.clone()); // Clone the sequence to own it
        }
    }

    (result, invalid_sequences)
}

fn is_valid_seq(rules: &Vec<(i32, i32)>, seq: &Vec<i32>) -> bool {
    // Check the sequence for validity based on the rules
    rules.iter().all(|&(a, b)| {
        if let (Some(a_idx), Some(b_idx)) = (
            seq.iter().position(|&x| x == a),
            seq.iter().position(|&x| x == b),
        ) {
            a_idx < b_idx
        } else {
            true
        }
    })
}

fn run_2(rules: &Vec<(i32, i32)>, invalid_sequences: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;

    for seq in invalid_sequences {
        let mut sorted_seq = seq.clone();

        // Sort the sequence using a custom comparator
        sorted_seq.sort_by(|&p, &q| {
            if rules.iter().any(|&(a, b)| a == p && b == q) {
                std::cmp::Ordering::Less
            } else if rules.iter().any(|&(a, b)| a == q && b == p) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });

        if !sorted_seq.is_empty() {
            let mid_index = (sorted_seq.len() - 1) / 2;
            result += sorted_seq[mid_index];
        }
    }

    result
}

fn main() -> io::Result<()> {
    let file_path = "src/05/input.txt";

    let (rules, sequences) = read_file(file_path).unwrap();

    let (result_first_quiz, invalid_sequences) = run_1(&rules, &sequences);

    println!("{}", result_first_quiz);

    let result_second_quiz = run_2(&rules, &invalid_sequences);

    println!("{}", result_second_quiz);

    Ok(())
}
