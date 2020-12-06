pub fn call(puzzle_input: String) {

    let map: Vec<Vec<char>> = puzzle_input.lines().map(|x| x.chars().collect()).collect();

    let single_slope_count = calculate_tree_to_slope_ratio(3, 1, &map);

    let slopes = [
        [1,1],
        [5,1],
        [7,1],
        [1,2]
    ];

    let multi_slope_count = slopes.iter().fold(1, |acc, slope|{
        acc * calculate_tree_to_slope_ratio(slope[0], slope[1], &map)
    });

    println!("Tree count A: {}", single_slope_count);
    println!("Tree count B: {}", single_slope_count * multi_slope_count);
}

fn calculate_tree_to_slope_ratio(across: usize, down: usize, map: &Vec<Vec<char>>) -> usize {
    let mut current_index_right = 0;
    let mut tree_count = 0;

    let map_iterator = map.iter().step_by(down);
    for line in map_iterator {
        let length = line.len();

        let position = line[current_index_right % length];
        match position {
            '#' => tree_count += 1,
            '.' => (),
            _ => println!("Unknown map symbol.")
        }

        current_index_right += across;
    }

    tree_count
}
