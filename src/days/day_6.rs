
pub fn call(puzzle_input: String) {

    // Take the string and split into the groups
    let raw_groups = puzzle_input.split("\n\n");

    let sum = raw_groups.fold(0, |acc, group| {
        acc + count_unique_values(group)
    });

    println!("Sum of groups: {}", sum);
}

fn count_unique_values(group: &str) -> usize {
    let mut chars: Vec<char> = group.split_whitespace().collect::<String>().chars().collect();
    chars.sort_unstable();
    chars.dedup();
    chars.len()
}
