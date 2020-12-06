
pub fn call(puzzle_input: String) {
    let password_entries = puzzle_input.lines();

    let policies: Vec<PasswordPolicy> = password_entries.map(|entry| {
        PasswordPolicy::new(entry)
    }).collect();

    // TODO: don't double go over
    let a_count = policies.iter().filter(|x| x.valid_a()).count();
    println!("Valid A passwords: {}", a_count);

    let b_count = policies.iter().filter(|x| x.valid_b()).count();
    println!("Valid B passwords: {}", b_count);
}

struct PasswordPolicy {
    lower_limit: usize,
    upper_limit: usize,
    letter: char,
    password: Vec<char>,
}

impl PasswordPolicy {
    fn new(entry: &str) -> PasswordPolicy{
        let parts: Vec<&str> = entry.split(":").collect();
        let limits_and_letter: Vec<&str> = parts[0].split(" ").collect();
        let limits: Vec<usize> = limits_and_letter[0].split("-").map(|x| x.parse::<usize>().unwrap()).collect();
        let letter = limits_and_letter[1];
        let chars: Vec<char> = parts[1].chars().collect();

        PasswordPolicy {
            lower_limit: limits[0],
            upper_limit: limits[1],
            letter: letter.chars().next().unwrap(),
            password: chars
        }
    }

    fn valid_a(&self) -> bool {
        let letter_count = self.password.iter().filter(|&c| *c == self.letter).count();
        letter_count >= self.lower_limit && letter_count <= self.upper_limit
    }

    fn valid_b(&self) -> bool {
        (self.password[self.lower_limit] == self.letter) ^ (self.password[self.upper_limit] == self.letter)
    }
}
