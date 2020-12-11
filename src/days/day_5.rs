
pub fn call(puzzle_input: String) {
    let mut highest_seat_id = 0;
    let mut lowest_seat_id = 60;
    let mut sum_of_seats: u32 = 0;

    puzzle_input.lines().for_each(|x| {
        let pass: Vec<char> = x.chars().collect();
        let seat_id = shift_bits_for_chars(&pass[..]);

        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        } else if seat_id < lowest_seat_id {
            lowest_seat_id = seat_id
        }

        sum_of_seats += seat_id as u32;
    });

    // assuming contigious values with gaps at top and bottom - we can just sum and know the missing value.
    // we can use n×(n+1)/2 to get the sum from 0..n and subtract the missing vals from the bottom 0..lowest_seat
    let total_sum_no_missing = highest_seat_id * (highest_seat_id+1) / 2;
    let lower_missing: u32 = (0..lowest_seat_id).sum();

    let our_seat = total_sum_no_missing - lower_missing - sum_of_seats;

    println!("Highest seat_id: {}", highest_seat_id);
    println!("Our seat: {}", our_seat);
}

fn shift_bits_for_chars(bits: &[char]) -> u32 {
    let mut converted_row = 0;
    for bit in bits {
        converted_row <<= 1;

        if bit == &'B' || bit == &'R' {
            converted_row |= 0b0000_0000_0001;
        }
    }

    converted_row
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_shifts_bits_based_on_the_string() {
        let some_chars: Vec<char> = "BFFFBBFRLL".chars().collect();
        let row_result = shift_bits_for_chars(&some_chars[..]);
        assert_eq!(row_result, 564);
    }

}
