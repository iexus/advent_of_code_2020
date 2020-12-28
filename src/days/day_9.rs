// find the first number in the list (after the preamble)
// which is not the sum of two of the 25 numbers before it.

pub fn call(puzzle_input: String) {
    let lines = puzzle_input.lines();
    let numbers: Vec<usize> = lines.map(|entry| {
        entry.parse().unwrap()
    }).collect();

    // Preamble of 25 numbers.
    let mut current_set;
    let mut index = 25;

    loop {
        if index >= numbers.len() {
            println!("Reached end of the array without finding invalid number");
        }

        let next_num = &numbers[index];
        current_set = &numbers[index-25..index];

        let mut match_found = false;
        for test_num in current_set {
            // can ignore if it's >= our next_num
            if test_num >= next_num {
                continue;
            }

            // else we can subtract from the target and see if the result
            // is contained in the list
            let target = next_num - test_num;
            if current_set.contains(&target) {
                match_found = true;
                break;
            }
        }

        if !match_found {
            println!("Did not find a match for number: {}, it cannot be valid", next_num);
            break;
        }

        index += 1;
    }
}
