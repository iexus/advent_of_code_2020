use std::collections::HashMap;

pub fn call(puzzle_input: String) {

    // Take the string and split into the groups
    let raw_groups = puzzle_input.split("\n\n");

    let sum_a = raw_groups.clone().fold(0, |acc, group| {
        acc + count_unique_values(group)
    });

    let sum_b = raw_groups.fold(0, |acc, group| {
        acc + count_group_selected_values(group)
    });

    println!("Sum of groups: {}, {}", sum_a, sum_b);
}

fn count_unique_values(group: &str) -> usize {
    let mut chars: Vec<char> = group.split_whitespace().collect::<String>().chars().collect();
    chars.sort_unstable();
    chars.dedup();
    chars.len()
}

fn count_group_selected_values(group: &str) -> u32 {
    // Number of lines in group = number of people
    let lines = group.lines();
    let num_people = lines.clone().collect::<Vec<&str>>().len();

    let no_whitespace = lines.collect::<String>();
    let mut hash = HashMap::<char, u32>::new();

    for single_char in no_whitespace.chars() {
        let value = hash.entry(single_char).or_insert(0);
        *value += 1;
    }

    let mut count = 0;
    for (_, val) in hash.iter() {
        if *val as usize >= num_people {
            count += 1;
        }
    }

    count
}
