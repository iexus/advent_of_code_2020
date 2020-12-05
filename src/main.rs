use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let file_name = "./puzzle_input/day_1.txt";
    let puzzle_input = fs::read_to_string(file_name)
        .expect("Error loading the test input");

    let puzzle_lines: Vec<i32> = puzzle_input.lines().map(|x| x.parse::<i32>().unwrap()).collect();

    // match find_matching_numbers(puzzle_lines) {
    //     Some(result) => println!("Result: {}", result),
    //     None => println!("Could not find a matching series")
    // }

    match search_with_lookup(puzzle_lines) {
        Some(result) => println!("Result: {}", result),
        None => println!("Could not find a matching series")
    }
}

fn find_matching_numbers(puzzle_lines: Vec<i32>) -> Option<i32> {
    let size = puzzle_lines.len();

    for i in 0..size {
        let val1 = puzzle_lines.get(i).unwrap();

        for j in i+1..size {
            let val2 = puzzle_lines.get(j).unwrap();

            for k in j+1..size {
                let val3 = puzzle_lines.get(k).unwrap();
                if val1 + val2 + val3 == 2020 {
                    return Some(val1 * val2 * val3);
                }
            }
        }
    }

    return None;
}

fn search_with_lookup(puzzle_lines: Vec<i32>) -> Option<i32> {
    let values = HashSet::<i32>::from_iter(puzzle_lines.iter().cloned());

    let size = puzzle_lines.len();
    for i in 0..size {
        let val1 = puzzle_lines.get(i).unwrap();

        for j in i+1..size {
            let val2 = puzzle_lines.get(j).unwrap();
            let missing = 2020 - val1 - val2;

            if values.contains(&missing) {
                return Some(val1 * val2 * missing);
            }
        }
    }

    return None;
}
