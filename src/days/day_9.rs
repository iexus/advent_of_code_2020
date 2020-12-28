
pub fn call(puzzle_input: String) {
    let lines = puzzle_input.lines();
    let numbers: Vec<usize> = lines.map(|entry| {
        entry.parse().unwrap()
    }).collect();

    // Part A
    let invalid = find_invalid_num(&numbers);

    // Part B
    match find_weakness_number(&numbers, invalid) {
        Some(val) => println!("Weakness: {}", val),
        None => println!("Could not find the weakness"),
    }
}

fn find_weakness_number(numbers: &Vec<usize>, invalid: usize) -> Option<usize> {
    let mut index = 0;
    loop {
        if index >= numbers.len() {
            return None;
        }

        // start at index and keep summing until it equals or is over
        // IF over then start again (can't be too clever here as numbers aren't in order?)
        match find_contigious_numbers_from_index(numbers, index, invalid) {
            Some(mut result) => {
                // need to add the largest and the smallest results
                result.sort();
                return Some(result.first()? + result.last()?);
            },
            None => index += 1,
        }
    }
}

fn find_contigious_numbers_from_index(numbers: &Vec<usize>, index: usize, invalid: usize) -> Option<Vec<usize>>{
    let mut sum = 0;
    let mut current_pos = index;
    for test_num in &numbers[index..] {
        sum += test_num;

        if sum == invalid {
            return Some((&numbers[index..=current_pos]).to_vec());
        }
        else if sum > invalid {
            return None;
        }

        current_pos += 1;
    }

    None
}

fn find_invalid_num(numbers: &Vec<usize>) -> usize {
    // find the first number in the list (after the preamble)
    // which is not the sum of two of the 25 numbers before it.
    // Preamble is 25 numbers.
    let mut current_set;
    let mut index = 25;

    loop {
        if index >= numbers.len() {
            println!("Reached end of the array without finding invalid number");
        }

        let next_num = &numbers[index];
        current_set = &numbers[index-25..index];

        match check_if_valid_number(current_set, next_num) {
            true => index += 1,
            false => {
                println!("Did not find a match for number: {}, it cannot be valid", next_num);
                return *next_num;
            }

        }
    }
}

fn check_if_valid_number(current_set: &[usize], next_num: &usize) -> bool {
    for test_num in current_set {
        // can ignore if it's >= our next_num
        if test_num >= next_num {
            continue;
        }

        // else we can subtract from the target and see if the result
        // is contained in the list
        let target = next_num - test_num;
        if current_set.contains(&target) {
            return true;
        }
    }

    false
}
