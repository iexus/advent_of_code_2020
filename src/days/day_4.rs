use std::result;

type Result<T> = result::Result<T, PassportError>;

#[derive(Debug, Clone)]
struct PassportError {
    message: String,
}

pub fn call(puzzle_input: String) {
    let raw_passports: Vec<&str> = puzzle_input.split("\n\n").collect();

    let results: Vec<u8> = raw_passports.iter()
        .map(|pass| parse_passport(pass))
        .filter_map(Result::ok)
        .collect();

    println!("Success count: {}", results.len());
}

fn parse_passport(entry: &str) -> Result<u8> {
    let parts: Vec<&str> = entry.split_whitespace().collect();

    match parts.len() {
        8 => return Ok(8),
        7 => return check_for_cid(parts),
        _ => return Err(PassportError { message: String::from("Missing many fields") })
    }
}

fn check_for_cid(parts: Vec<&str>) -> Result<u8> {
    for part in &parts {
        if part.contains("cid") {
            return Err(PassportError { message: String::from("Missing a field other than CID") });
        }
    }

    Ok(7)
}
