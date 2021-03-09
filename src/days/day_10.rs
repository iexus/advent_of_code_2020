use itertools::Itertools;

pub fn call(puzzle_input: String) {
    // Part A - count the 1 jolt differences and multiply by 3 jolt differences
    // Order all the adapters
    let mut adapters: Vec<i32> = puzzle_input.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    adapters.sort();

    // Push in the first adapter (which is 0)
    adapters.insert(0, 0);

    // Part A
    find_jolt_totals(&adapters);

    // Part B
    find_total_combos(&adapters);
}

fn find_jolt_totals(adapters: &Vec<i32>) {
    // We need to go over the whole list comparing pairs to see the min-diff
    let mut jolt_1 = 0;
    let mut jolt_3 = 0;
    for (a,b) in adapters.iter().tuple_windows() {
        match b - a {
            1 => jolt_1 += 1,
            3 => jolt_3 += 1,
            _ => ()
        }
    }

    // We need to consider the last thing which is your phone which is always 3 higher.
    jolt_3 += 1;

    println!("Part A: {}", jolt_1 * jolt_3);
}

fn find_total_combos(adapters: &Vec<i32>) -> u128 {
    let length = adapters.len();

    // It's easier if we do this backwards
    let mut reversed_adapters = adapters.clone();
    reversed_adapters.reverse();

    // We know we hit each item once in the longest combo
    let mut visits: Vec<u128> = vec![0; length];
    for (i, adapter) in reversed_adapters.iter().enumerate() {
        if visits[i] == 0 {
            visits[i] = 1;
        }

        // We can always reach the next entry (as long as there's more to go)
        let current_visits = visits[i];
        if i < length - 1 {
            visits[i+1] += current_visits;
        }

        // try the next 2 places out - if we can reach add ourselves to them
        if (i < length - 2) && adapter - reversed_adapters[i + 2] <= 3 {
            visits[i+2] += current_visits;
        }
        if (i < length - 3) && adapter - reversed_adapters[i + 3] <= 3 {
            visits[i+3] += current_visits;
        }
    }

    let result = visits[length - 1];

    println!("Final vists: {}", result);
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_sums_up_visits_correctly() {
        let mut input = vec![0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19];
        let result = find_total_combos(&mut input);
        assert_eq!(result, 8);

        let mut input = vec![
            0, 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19,
            20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38, 39,
            42, 45, 46, 47, 48, 49
        ];

        let result = find_total_combos(&mut input);
        assert_eq!(result, 19208);
    }
}
