extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

const REGEX_MUL: &str = r"mul\(\d{1,3},\d{1,3}\)";
const REGEX_MU_DO_DONT: &str = r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)";

fn get_product(mul_as_str: &str) -> i32 {
    // Find the substring between '(' and ')', then split by comma and parse to integers
    let start = mul_as_str.find('(').unwrap() + 1;
    let end = mul_as_str.find(')').unwrap();
    let numbers: Vec<i32> = mul_as_str[start..end]
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    numbers[0] * numbers[1]
}

fn read_string(file_path: &str) -> io::Result<String> {
    let mut input_string = String::new();

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        input_string.push_str(&line);
    }

    Ok(input_string)
}

fn run_1(input_string: &str) -> i32 {
    let re = Regex::new(REGEX_MUL).unwrap();
    let mut result = 0;

    for item in re.find_iter(input_string) {
        let mul_as_str = item.as_str();
        result += get_product(mul_as_str);
    }

    result
}

fn run_2(input_string: &str) -> i32 {
    let re = Regex::new(REGEX_MU_DO_DONT).unwrap();
    let mut result = 0;
    let mut enabled = true;

    for item in re.find_iter(input_string) {
        let matched = item.as_str();

        if matched.contains("do()") {
            enabled = true;
        } else if matched.contains("don't()") {
            enabled = false;
        } else if enabled {
            result += get_product(matched);
        }
    }
    result
}

fn main() -> io::Result<()> {
    let file_path = "src/03/input.txt";

    let input_string = read_string(file_path)?;

    let result_first_quiz = run_1(&input_string);

    println!("{}", result_first_quiz);

    let result_second_quiz = run_2(&input_string);

    println!("{}", result_second_quiz);

    Ok(())
}
