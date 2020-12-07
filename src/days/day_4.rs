use std::result;
use std::num::ParseIntError;
use regex::Regex;

const HEIGHT_LIMIT_INCHES: (u8, u8) = (59, 76);
const HEIGHT_LIMIT_CM: (u8, u8) = (150, 193);

type Result<T> = result::Result<T, PassportError>;

#[derive(Debug, Clone)]
struct PassportError {
    message: String,
}

impl From<ParseIntError> for PassportError {
    fn from(err: ParseIntError) -> Self {
        PassportError { message: format!("{}", err) }
    }
}

pub fn call(puzzle_input: String) {
    let raw_passports: Vec<&str> = puzzle_input.split("\n\n").collect();

    let count = raw_passports.iter()
        .map(|pass| parse_passport(pass))
        .filter_map(Result::ok)
        .count();

    println!("Success count: {}", count);
}

fn parse_passport(entry: &str) -> Result<()> {
    let parts: Vec<&str> = entry.split_whitespace().collect();

    match parts.len() {
        8 => return validate_fields(parts, false),
        7 => return validate_fields(parts, true),
        _ => return Err(PassportError { message: String::from("Missing many fields") })
    };
}

fn validate_fields(fields: Vec<&str>, cid_check: bool) -> Result<()> {
    for field in &fields {
        let field_parts: Vec<&str> = field.split(":").collect();
        let token = field_parts[0];
        let value = field_parts[1];

        match token {
            "cid" => {
                if cid_check {
                    return Err(PassportError { message: String::from("Missing a field other than CID") });
                }
            },
            "byr" => validate_year(value, 1920, 2002)?,
            "iyr" => validate_year(value, 2010, 2020)?,
            "eyr" => validate_year(value, 2020, 2030)?,
            "hgt" => validate_height(value)?,
            "hcl" => validate_pattern(value, &Regex::new(r"#[a-f|\d]{6}").unwrap())?,
            "ecl" => validate_pattern(value, &Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap())?,
            "pid" => validate_passport(value)?,
            _ => return Err(PassportError { message: String::from("Could not match a field")})
        }
    }

    Ok(())
}

fn validate_year(raw_year: &str, start_date: u32, end_date: u32) -> Result<()> {
    let parsed_year: u32 = raw_year.parse::<u32>()?;
    if parsed_year < start_date || parsed_year > end_date {
        return Err(PassportError { message: format!("{}, was not in date range {} to {}", raw_year, start_date, end_date) });
    }

    Ok(())
}

fn validate_height(raw_height: &str) -> Result<()> {
    let split_position = raw_height.len() - 2;
    let unit = &raw_height[split_position..];
    let height = (&raw_height[..split_position]).parse::<u8>()?;

    let limits = match unit {
        "cm" => HEIGHT_LIMIT_CM,
        "in" => HEIGHT_LIMIT_INCHES,
        _ => return Err(PassportError { message: format!("{} is not a valid unit for height", unit) })
    };

    if height < limits.0 || height > limits.1 {
        return Err(PassportError { message: format!("{}, was not in height range {} to {}", height, limits.0, limits.1) });
    }

    Ok(())
}

fn validate_pattern(raw_string: &str, pattern: &regex::Regex) -> Result<()> {
    if pattern.is_match(raw_string) {
        return Ok(())
    }

    return Err(PassportError { message: format!("{}, was not found in string: {}", pattern, raw_string)});
}

fn validate_passport(raw_pass: &str) -> Result<()> {
    if raw_pass.len() != 9 || raw_pass.parse::<u32>().is_err() {
        return Err(PassportError { message: format!("{} is not a valid passport number", raw_pass)});
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_validates_years() {
        assert!(validate_year("1980", 1980, 2000).is_ok());
        assert!(validate_year("2000", 1980, 2000).is_ok());

        assert!(validate_year("1979", 1980, 2000).is_err());
        assert!(validate_year("2001", 1980, 2000).is_err());
    }

    #[test]
    fn it_validates_pattern_hair() {
        let pattern = Regex::new(r"#[a-f|\d]{6}").unwrap();

        assert!(validate_pattern("#1aff99", &pattern).is_ok());

        assert!(validate_pattern("#gggggg", &pattern).is_err());
        assert!(validate_pattern("1aff99", &pattern).is_err());
        assert!(validate_pattern("1111111", &pattern).is_err());
    }

    #[test]
    fn it_validates_pattern_eyes() {
        let pattern = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap();

        assert!(validate_pattern("amb", &pattern).is_ok());
        assert!(validate_pattern("nope", &pattern).is_err());
    }

    #[test]
    fn it_validates_heights() {
        assert!(validate_height("150cm").is_ok());
        assert!(validate_height("193cm").is_ok());
        assert!(validate_height("59in").is_ok());
        assert!(validate_height("76in").is_ok());

        assert!(validate_height("149cm").is_err());
        assert!(validate_height("194cm").is_err());
        assert!(validate_height("58in").is_err());
        assert!(validate_height("77in").is_err());
    }

    #[test]
    fn it_validates_passports() {
        assert!(validate_passport("000000001").is_ok());
        assert!(validate_passport("0123456789").is_err());
    }
}
