
pub fn call(puzzle_input: String) {
    let boarding_passes: Vec<Vec<char>> = puzzle_input.lines()
        .map(|x| x.chars().collect())
        .collect();

    let mut highest_seat_id = 0;
    let mut lowest_seat_id = 60;
    let mut sum_of_seats: u32 = 0;

    for pass in boarding_passes {
        let instructions = split_instructions(&pass);

        let row_value = shift_bits_for_chars(instructions.0, &'B');
        let col_value = shift_bits_for_chars(instructions.1, &'R');

        let seat_id: u16 = (row_value * 8) + col_value;

        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        } else if seat_id < lowest_seat_id {
            lowest_seat_id = seat_id
        }

        sum_of_seats += seat_id as u32;
    }

    // assuming contigious values with gaps at top and bottom - we can just sum and know the missing value.
    // This expected total could be cleverer I think: n×(n+1)/2 and subtract the missing vals from top and bottom?
    let expected_total: u32 = (lowest_seat_id..=highest_seat_id).fold(0, |acc, val| acc + val as u32);
    let our_seat = expected_total - sum_of_seats;

    println!("Highest seat_id: {}", highest_seat_id);
    println!("Our seat: {}", our_seat);
}

fn shift_bits_for_chars(bits: &[char], mask: &char) -> u16 {
    let mut converted_row = 0;
    for bit in bits {
        converted_row <<= 1;

        if bit == mask {
            converted_row |= 0b0000_0001;
        }
    }

    converted_row
}

fn split_instructions(pass: &Vec<char>) -> (&[char], &[char]) {
    (&pass[..7], &pass[7..])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_splits_boarding_passes_correctly() {
        let pass = "BFFBFBBLRL".chars().collect();
        let expected_start: Vec<char> = "BFFBFBB".chars().collect();
        let expected_end: Vec<char> = "LRL".chars().collect();

        let instructions = split_instructions(&pass);
        assert_eq!(instructions.0, &expected_start[..]);
        assert_eq!(instructions.1, &expected_end[..]);
    }

    #[test]
    fn it_shifts_bits_based_on_the_string() {
        let some_chars: Vec<char> = "BFFFBBF".chars().collect();
        let row_result = shift_bits_for_chars(&some_chars[..], &'B');
        assert_eq!(row_result, 70);

        let some_other_chars: Vec<char> = "RLL".chars().collect();
        let column_result = shift_bits_for_chars(&some_other_chars[..], &'R');
        assert_eq!(column_result, 4);
    }

}
