use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

const NUMBER_BLINKS_PART_1: u64 = 25;
const NUMBER_BLINKS_PART_2: u64 = 75;


fn read_int_list(filename: &str) -> Result<Vec<u64>, io::Error> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut input_list = Vec::new();

    for line in reader.lines() {
        let line = line?;
        input_list.extend(
            line.split_whitespace()
                .filter_map(|word| word.parse::<u64>().ok())
        );
    }

    Ok(input_list)
}


fn run(stones: &Vec<u64>, max_num_blinks: u64) -> u64 {
    stones
        .iter()
        .map(|&stone| {
            let mut memo = HashMap::new();
            count_stones(stone, max_num_blinks, &mut memo)
        })
        .sum()
}

fn process_stone(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }

    let stone_as_str = stone.to_string();
    let len = stone_as_str.len();

    if len % 2 == 0 {
        let half = len / 2;
        let first_half = stone_as_str[..half].parse::<u64>().unwrap();
        let second_half = stone_as_str[half..].parse::<u64>().unwrap();
        vec![first_half, second_half]
    } else {
        vec![stone * 2024]
    }
}

fn count_stones(value: u64, count: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    // Base case
    if count == 0 {
        return 1;
    }

    // Cache check
    let key = (value, count);
    if let Some(&result) = memo.get(&key) {
        return result;
    }

    // Recursive calculation
    let next_step_stones = process_stone(value);
    let result = next_step_stones
        .iter()
        .map(|&stone| count_stones(stone, count - 1, memo))
        .sum();

    // Store in memo
    memo.insert(key, result);
    result
}

fn main() -> io::Result<()> {
    let file_path = "src/11/input.txt";
    let stones = read_int_list(file_path).unwrap();

    let result_first_quiz = run(&stones, NUMBER_BLINKS_PART_1);
    println!("{}", result_first_quiz);

    let result_second_quiz = run(&stones, NUMBER_BLINKS_PART_2);
    println!("{}", result_second_quiz);

    Ok(())
}