
use std::fs::File;
use std::io::{self, BufRead};

fn read_string(filename: &str) -> io::Result<Vec<char>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let input_string: Vec<char> = reader
        .lines()
        .filter_map(Result::ok) // Read each line and handle errors
        .flat_map(|line| line.chars().collect::<Vec<char>>()) // Collect characters into Vec<char>
        .collect();

    Ok(input_string)
}

fn run_1(compact_disk: &Vec<char>) -> i32 {
    let expanded_disk = expand_disk_map(compact_disk);
    let optimized_disk = optimize_disk_map(&expanded_disk);
    compute_checksum(&optimized_disk)
}

fn expand_disk_map(compressed_disk: &Vec<char>) -> Vec<char> {
    let mut expanded = Vec::new();
    let mut current_id: u32 = 0;
    let mut space_part = false;

    for &item in compressed_disk {
        let count: usize = item.to_digit(10).unwrap() as usize; // Parse the char as a digit
        if space_part {
            expanded.extend(vec!['.'; count]); // Use `.` as a char directly
        } else {
            let id_char = char::from_digit(current_id as u32, 10);
            expanded.extend(vec![id_char; count]); // Add the ID as a char
            current_id += 1;
        }
        space_part = !space_part;
    }

    expanded
}

fn optimize_disk_map(expanded_disk: &Vec<char>) -> Vec<char> {
    let mut expanded_disk = expanded_disk.clone();
    let mut left = 0;
    let mut right = expanded_disk.len() - 1;

    while left < right {
        while left < expanded_disk.len() && expanded_disk[left] != '.' {
            left += 1;
        }
        while right > 0 && expanded_disk[right] == '.' {
            right -= 1;
        }

        if left < right {
            expanded_disk[left] = expanded_disk[right];
            expanded_disk[right] = '.';
            left += 1;
            right -= 1;
        }
    }

    expanded_disk
}

fn compute_checksum(optimized_disk: &Vec<char>) -> i32 {
    let valid: Vec<&char> = optimized_disk.iter().filter(|&&e| e != '.').collect();
    valid
        .iter()
        .enumerate()
        .fold(0, |acc, (index, &id)| {
            acc + index as i32 * id.to_digit(10).unwrap() as i32
        })
}


fn main() -> io::Result<()> {
    let file_path = "src/09/input.txt";

    let input_string = read_string(file_path).unwrap();

    let result_first_quiz = run_1(&input_string);
    println!("{}", result_first_quiz);

    /*
    let result_second_quiz = run_2(&antenna_positions, &matrix);
    println!("{}", result_second_quiz);
    */

    Ok(())
}