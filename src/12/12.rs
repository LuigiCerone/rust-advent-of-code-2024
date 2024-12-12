use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::collections::{HashMap, HashSet};

type Point = (usize, usize);

const ALLOWED_DIRECTIONS: [(isize, isize, &str); 4] =
    [(-1, 0, "U"), (1, 0, "D"), (0, -1, "L"), (0, 1, "R")];

fn read_matrix(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let lines = io::BufReader::new(file).lines();

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let line = line?;
        let values: Vec<char> = line.trim().chars().collect();
        matrix.push(values);
    }

    Ok(matrix)
}

fn is_in_matrix(point: Point, rows: usize, cols: usize) -> bool {
    let (x, y) = point;
    x < rows && y < cols
}

fn find_area_points(
    matrix: &Vec<Vec<char>>,
    r: usize,
    c: usize,
    visited_set: &mut HashSet<Point>,
    curr_area: &mut Vec<Point>,
) -> Vec<Point> {
    visited_set.insert((r, c));

    for (dr, dc, _direction) in ALLOWED_DIRECTIONS {
        let nr = r as isize + dr;
        let nc = c as isize + dc;

        if nr >= 0 && nc >= 0 {
            let nr = nr as usize;
            let nc = nc as usize;

            if !visited_set.contains(&(nr, nc))
                && is_in_matrix((nr, nc), matrix.len(), matrix[0].len())
                && matrix[r][c] == matrix[nr][nc]
            {
                curr_area.push((nr, nc));
                find_area_points(matrix, nr, nc, visited_set, curr_area);
            }
        }
    }

    curr_area.clone()
}

fn find_all_areas(matrix: &Vec<Vec<char>>) -> HashMap<char, Vec<Vec<Point>>> {
    let mut visited_set = HashSet::new();
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut areas: HashMap<char, Vec<Vec<Point>>> = HashMap::new();

    for r in 0..rows {
        for c in 0..cols {
            if !visited_set.contains(&(r, c)) {
                let mut new_area = vec![(r, c)];
                let area_points = find_area_points(matrix, r, c, &mut visited_set, &mut new_area);
                areas
                    .entry(matrix[r][c])
                    .or_insert(vec![])
                    .push(area_points);
            }
        }
    }

    areas
}

fn compute_perimeter(area: &Vec<Point>, rows: usize, cols: usize) -> usize {
    let mut perimeter = 0;
    let area_set: HashSet<Point> = area.iter().cloned().collect();

    for &(r, c) in area {
        for (dr, dc, _direction) in ALLOWED_DIRECTIONS {
            let nr = r as isize + dr;
            let nc = c as isize + dc;

            if !is_in_matrix((nr as usize, nc as usize), rows, cols) {
                perimeter += 1;
            } else {
                let neighbor = (nr as usize, nc as usize);
                if !area_set.contains(&neighbor) {
                    perimeter += 1;
                }
            }
        }
    }

    perimeter
}

fn compute_sides(area: &Vec<(usize, usize)>) -> usize {
    let area_set: HashSet<_> = area.iter().cloned().collect();

    let mut left = HashSet::new();
    let mut right = HashSet::new();
    let mut up = HashSet::new();
    let mut down = HashSet::new();

    for &(r, c) in area {
        for &(dr, dc, direction) in &ALLOWED_DIRECTIONS {
            let neighbor = (r as isize + dr, c as isize + dc);
            if !area_set.contains(&(neighbor.0 as usize, neighbor.1 as usize)) {
                match direction {
                    "U" => {
                        up.insert((r, c));
                    }
                    "D" => {
                        down.insert((r, c));
                    }
                    "L" => {
                        left.insert((r, c));
                    }
                    "R" => {
                        right.insert((r, c));
                    }
                    _ => (),
                }
            }
        }
    }

    let mut corners = 0;

    for &(r, c) in &up {
        // Check if the neighbor coordinates are valid before using them
        let left_neighbor = (r as isize - 1, c as isize - 1);
        let right_neighbor = (r as isize - 1, c as isize + 1);
        if area_set.contains(&(left_neighbor.0 as usize, left_neighbor.1 as usize))
            && !left.contains(&(r, c))
        {
            corners += 1;
        }
        if area_set.contains(&(right_neighbor.0 as usize, right_neighbor.1 as usize))
            && !right.contains(&(r, c))
        {
            corners += 1;
        }
    }

    for &(r, c) in &down {
        // Check if the neighbor coordinates are valid before using them
        let left_neighbor = (r as isize + 1, c as isize - 1);
        let right_neighbor = (r as isize + 1, c as isize + 1);
        if area_set.contains(&(left_neighbor.0 as usize, left_neighbor.1 as usize))
            && !left.contains(&(r, c))
        {
            corners += 1;
        }
        if area_set.contains(&(right_neighbor.0 as usize, right_neighbor.1 as usize))
            && !right.contains(&(r, c))
        {
            corners += 1;
        }
    }

    corners
}

fn run(matrix: &Vec<Vec<char>>, part_1: bool) -> usize {
    let areas = find_all_areas(&matrix);
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut result = 0;

    for plant_areas in areas.values() {
        for plant_area in plant_areas {
            let area = plant_area.len();

            if part_1 {
                let perimeter = compute_perimeter(plant_area, rows, cols);
                result += area * perimeter;
            } else {
                let num_sides = compute_sides(plant_area);
                result += area * num_sides;
            }
        }
    }

    result
}

fn main() -> io::Result<()> {
    let file_path = "src/12/input.txt";

    let matrix = read_matrix(file_path).unwrap();
    // println!("{:?}", matrix);

    let result_first_quiz = run(&matrix, true);
    println!("{}", result_first_quiz);

    let result_second_quiz = run(&matrix, false);
    println!("{}", result_second_quiz);

    Ok(())
}
