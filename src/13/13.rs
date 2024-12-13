use std::fs;
use std::error::Error;
use nalgebra::{Matrix2, Vector2};
use regex::Regex;

const A_PRICE: i32 = 3;
const B_PRICE: i32 = 1;
const PART_2_INCREASE: i64 = 10000000000000;

#[derive(Debug)]
struct Solution {
    solvable: bool,
    k: Option<i32>,
    m: Option<i32>,
}

#[derive(Debug, Clone)]
struct MachineInput {
    a: (i32, i32),
    b: (i32, i32),
    p: (i32, i32),
}

fn read_machines(file_path: &str) -> Result<Vec<MachineInput>, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    let button_pattern = Regex::new(r"Button (\w): X\+(\d+), Y\+(\d+)")?;
    let prize_pattern = Regex::new(r"Prize: X=(\d+), Y=(\d+)")?;
    let unparsed_machines: Vec<&str> = contents.split("\n\n").collect();

    let mut input_list = Vec::new();

    for um in unparsed_machines {
        let buttons: Vec<_> = button_pattern.captures_iter(um)
            .map(|cap| {
                let button = cap[1].to_lowercase();
                let x = cap[2].parse::<i32>().unwrap();
                let y = cap[3].parse::<i32>().unwrap();
                (button, (x, y))
            })
            .collect();

        let button_data: std::collections::HashMap<_, _> = buttons.into_iter().collect();

        let prize_data = if let Some(prize_match) = prize_pattern.captures(um) {
            Some((
                prize_match[1].parse::<i32>().unwrap(),
                prize_match[2].parse::<i32>().unwrap(),
            ))
        } else {
            None
        };

        if let Some(p) = prize_data {
            input_list.push(MachineInput {
                a: *button_data.get("a").unwrap_or(&(0, 0)),
                b: *button_data.get("b").unwrap_or(&(0, 0)),
                p,
            });
        }
    }

    Ok(input_list)
}

fn run(machines: Vec<MachineInput>, part_one: bool) -> i64 {
    let mut result = 0;

    for machine in machines {
        let mut p = (machine.p.0 as i64, machine.p.1 as i64);

        if !part_one {
            p.0 += PART_2_INCREASE;
            p.1 += PART_2_INCREASE;
        }

        let solution = find_solution(machine.a, machine.b, p);

        if !solution.solvable {
            continue;
        }

        if let (Some(k), Some(m)) = (solution.k, solution.m) {
            result += k as i64 * A_PRICE as i64 + m as i64 * B_PRICE as i64;
        }
    }

    result
}

fn is_close_to_integer(value: f64, tolerance: f64) -> bool {
    (value - value.round()).abs() < tolerance
}

fn find_solution(a: (i32, i32), b: (i32, i32), p: (i64, i64)) -> Solution {
    let coeff_matrix = Matrix2::new(a.0 as f64, b.0 as f64, a.1 as f64, b.1 as f64);
    let p_vector = Vector2::new(p.0 as f64, p.1 as f64);

    match coeff_matrix.try_inverse() {
        Some(inv_matrix) => {
            let solution = inv_matrix * p_vector;
            let k = solution[0];
            let m = solution[1];

            if is_close_to_integer(k, 1e-4) && is_close_to_integer(m, 1e-4) {
                Solution {
                    solvable: true,
                    k: Some(k.round() as i32),
                    m: Some(m.round() as i32),
                }
            } else {
                Solution { solvable: false, k: None, m: None }
            }
        }
        None => Solution { solvable: false, k: None, m: None },
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path: &str = "src/13/input.txt";

    let input_machines = read_machines(file_path)?;

    let r1 = run(input_machines.clone(), true);
    println!("{}", r1);

    let r2 = run(input_machines, false);
    println!("{}", r2);

    Ok(())
}