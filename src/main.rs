use std::env;
use std::fs;

mod days;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg1 = &args[1];
    let puzzle_day = arg1.parse::<i32>()
        .expect(&format!("Could not parse puzzle day: {}, must be integer", arg1));

    let file_name = &format!("./input/day_{}.txt", puzzle_day);
    let puzzle_input = fs::read_to_string(file_name)
        .expect(&format!("Error loading the test input: {}, check it is in the input folder", file_name));

    match puzzle_day {
        1 => days::day_1::call(puzzle_input),
        2 => days::day_2::call(puzzle_input),
        3 => days::day_3::call(puzzle_input),
        4 => days::day_4::call(puzzle_input),
        5 => days::day_5::call(puzzle_input),
        6 => days::day_6::call(puzzle_input),
        7 => days::day_7::call(puzzle_input),
        _ => println!("There was no matching puzzle day - have you written it yet you lazy fuck?")
    }
}
