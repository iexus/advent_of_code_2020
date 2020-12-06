pub fn call(puzzle_input: String) {

    // down 1, right 3

    let lines: Vec<Vec<char>> = puzzle_input.lines().map(|x| x.chars().collect()).collect();

    let mut current_index_right = 0;
    let mut tree_count = 0;

    for line in lines {
        let length = line.len();

        let position = line[current_index_right % length];
        match position {
            '#' => tree_count += 1,
            '.' => (),
            _ => println!("Unknown map symbol.")
        }

        current_index_right += 3;
    }

    println!("Tree count: {}", tree_count);
}
