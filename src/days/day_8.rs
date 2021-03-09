use regex::Regex;

#[derive(Debug, Clone)]
struct Instruction {
    command: String,
    amount: isize,
    is_dirty: bool,
}

impl Instruction {
    fn new(command: String, amount: isize) -> Instruction {
        Instruction{
            command,
            amount,
            is_dirty: false
        }
    }

    fn make_dirty(&mut self) {
        self.is_dirty = true;
    }

    fn change_command(&mut self, new_command: String) {
        self.command = new_command;
    }
}

pub fn call(puzzle_input: String) {
    // For A just try running the default instructions
    let mut instructions = build_list_of_instructions(puzzle_input.clone());
    let result = run_instructions(&mut instructions);
    println!("Result for A: {}", result.0);

    // For B - we either brute force it or try to be more clever
    let b_result = brute_force_instruction(puzzle_input);
    println!("Result for B: {}", b_result);
}


fn brute_force_instruction(input: String) -> isize {
    let instructions = build_list_of_instructions(input);
    let mut possible_solutions: Vec<Vec<Instruction>> = Vec::new();

    // go through each instruction and for each nop and jmp
    // create a copy where that value is changed
    for (pos, inst) in instructions.iter().enumerate() {
        match &inst.command[..] {
            "jmp" => {
                let mut copy = instructions.clone();
                let inst_copy = copy.get_mut(pos).unwrap();
                inst_copy.change_command(String::from("nop"));

                possible_solutions.push(copy);
            },
            "nop" => {
                let mut copy = instructions.clone();
                let inst_copy = copy.get_mut(pos).unwrap();
                inst_copy.change_command(String::from("jmp"));

                possible_solutions.push(copy);
            }
            _ => (),
        }
    }

    for mut solution in possible_solutions {
        let result = run_instructions(&mut solution);
        if result.1 {
            return result.0;
        }
    }

    println!("Could not find a possible solution!");
    0
}

fn run_instructions(instructions: &mut Vec<Instruction>) -> (isize, bool) {
    let mut accumulator: isize = 0;
    let mut current_index = 0;

    loop {
        // Are we outside of the instructions? Then we've finished
        if current_index >= instructions.len() {
            return (accumulator, true)
        }

        // if dirty - been here before
        let instruction = instructions.get_mut(current_index).unwrap();
        if instruction.is_dirty {
            break;
        }

        // Now mark this as dirty
        instruction.make_dirty();
        match &instruction.command[..] {
            "acc" => {
                accumulator += instruction.amount;
                current_index += 1;
            },
            "jmp" => {
                // superhacky with isize usize fun
                let jump: isize = current_index as isize + instruction.amount;
                current_index = jump as usize;
            },
            "nop" => current_index += 1,
            _ => println!("Did not recognise instruction: {}", instruction.command)
        }
    }

    (accumulator, false)
}

fn build_list_of_instructions(puzzle_input: String) -> Vec<Instruction> {
    // Type, direction down?, amount, touched already?
    let mut instructions: Vec<Instruction> = Vec::new();
    let instruction_pattern = Regex::new(r"(?P<command>([a-z]{3})) (?P<amount>([+-]\d+))").unwrap();

    // Build the list of instructions
    for line in puzzle_input.lines() {
        match parse_instruction(&instruction_pattern, line) {
            Some(instruction) => instructions.push(instruction),
            None => {
                println!("Could not create instruction from: {}", line);
            }
        }
    }

    instructions
}

fn parse_instruction(pattern: &regex::Regex, line: &str) -> Option<Instruction> {
    match pattern.captures(line) {
        None => None,
        Some(capture) => {
            let command = capture.name("command")?.as_str();
            let amount: isize = capture.name("amount").unwrap().as_str().parse().unwrap();

            let instruction = Instruction::new(command.to_string(), amount);
            Some(instruction)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_accumulated_value_following_instructions() {
        let input = String::from("nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6");

        let mut instructions = build_list_of_instructions(input);
        let accumulator = run_instructions(&mut instructions);
        assert_eq!(accumulator, (5, false));
    }
}
