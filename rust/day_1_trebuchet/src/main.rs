#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;
use std::env;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    macro_rules! resource {
        ($fname:expr) => {
            Path::new(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../resources/",
                $fname
            ))
        };
    }

    #[test]
    fn test_actual_puzzle_input() {
        let file_path = resource!("day_1/puzzle_input.txt");
        let file_contents = fs::read_to_string(file_path)
            .expect(format!("Could not read file {}!", file_path.to_str().unwrap()).as_str());
        assert_eq!(get_calibration_value(&file_contents), 53855);
    }

    #[test]
    fn test_example_1() {
        let file_contents = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(get_calibration_value(&file_contents), 142);
    }

    #[test]
    fn test_example_2() {
        let file_contents = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!(get_calibration_value(&file_contents), 281);
    }

    #[test]
    fn test_get_first_digit() {
        assert_eq!(get_first_digit("1abc2"), 1);
        assert_eq!(get_first_digit("pqr3stu8vwx"), 3);
        assert_eq!(get_first_digit("a1b2c3d4e5f"), 1);
        assert_eq!(get_first_digit("treb7uchet"), 7);
        assert_eq!(get_first_digit("two1nine"), 2);
        assert_eq!(get_first_digit("eightwothree"), 8);
        assert_eq!(get_first_digit("abcone2threexyz"), 1);
        assert_eq!(get_first_digit("xtwone3four"), 2);
        assert_eq!(get_first_digit("4nineeightseven2"), 4);
        assert_eq!(get_first_digit("zoneight234"), 1);
        assert_eq!(get_first_digit("7pqrstsixteen"), 7);
    }

    #[test]
    fn test_get_last_digit() {
        assert_eq!(get_last_digit("1abc2"), 2);
        assert_eq!(get_last_digit("pqr3stu8vwx"), 8);
        assert_eq!(get_last_digit("a1b2c3d4e5f"), 5);
        assert_eq!(get_last_digit("treb7uchet"), 7);
        assert_eq!(get_last_digit("two1nine"), 9);
        assert_eq!(get_last_digit("eightwothree"), 3);
        assert_eq!(get_last_digit("abcone2threexyz"), 3);
        assert_eq!(get_last_digit("xtwone3four"), 4);
        assert_eq!(get_last_digit("4nineeightseven2"), 2);
        assert_eq!(get_last_digit("zoneight234"), 4);
        assert_eq!(get_last_digit("7pqrstsixteen"), 6);
        assert_eq!(get_last_digit("3tgsppcfpk"), 3);
    }
}
lazy_static! {
    static ref NUMBERS: HashMap<&'static str, u8> = HashMap::from([
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("ten", 10),
        ("eleven", 11),
        ("twelve", 12),
        ("thirteen", 13),
        ("fourteen", 14),
        ("fifteen", 15),
        ("sixteen", 16),
        ("seventeen", 17),
        ("eighteen", 18),
        ("nineteen", 19),
        ("twenty", 20),
        ("thirty", 30),
        ("forty", 40),
        ("fifty", 50),
        ("sixty", 60),
        ("seventy", 70),
        ("eighty", 80),
        ("ninety", 90),
        ("hundred", 100),
    ]);

    // The same as the other list, but with the string reversed so we can scan backwards
    static ref REVERSED_NUMBERS: HashMap<&'static str, u8> = HashMap::from([
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("orez", 0),
        ("eno", 1),
        ("owt", 2),
        ("eerht", 3),
        ("ruof", 4),
        ("evif", 5),
        ("xis", 6),
        ("neves", 7),
        ("thgie", 8),
        ("enin", 9),
        ("net", 10),
        ("nevele", 11),
        ("evlewot", 12),
        ("neetriht", 13),
        ("neetruof", 14),
        ("neetfif", 15),
        ("neetxis", 16),
        ("neetneves", 17),
        ("neethgie", 18),
        ("neetennin", 19),
        ("ytnewt", 20),
        ("ytneetriht", 30),
        ("ytneetruof", 40),
        ("ytneetfif", 50),
        ("ytneetxis", 60),
        ("ytneetneves", 70),
        ("ytneethgie", 80),
        ("ytneetennin", 90),
        ("derdnuh", 100),
    ]);
}

/// Get the first digit in a line of text
/// The first digit is the first digit in the line, or the first number word in the line
/// We will scan forward letter by letter until we find a digit or a number word
fn get_first_digit(line: &str) -> u32 {
    for i in 0..line.len() {
        let char = line.chars().nth(i).unwrap();
        let as_digit = char.to_digit(10);
        if as_digit.is_some() {
            return as_digit.unwrap();
        }
        let substring = &line[..i + 1];
        for (number_word, value) in NUMBERS.iter() {
            if substring.len() >= number_word.len() && substring.contains(number_word) {
                // only return the first digit
                return value
                    .to_string()
                    .chars()
                    .nth(0)
                    .unwrap()
                    .to_digit(10)
                    .unwrap();
            }
        }
    }
    0
}

/// Get the last digit in a line of text
/// The last digit is the last digit in the line, or the last number word in the line
/// We will scan backwards letter by letter until we find a digit or a number word
fn get_last_digit(line: &str) -> u32 {
    for i in (0..line.len()).rev() {
        let char = line.chars().nth(i).unwrap();
        let as_digit = char.to_digit(10);
        if as_digit.is_some() {
            return as_digit.unwrap();
        }
        // Make sure to reverse the string so we can check backwards!
        let substring = line[i..].chars().rev().collect::<String>();
        for (number_word, value) in REVERSED_NUMBERS.iter() {
            if substring.len() >= number_word.len() && substring.contains(number_word) {
                // only return the last digit
                return value
                    .to_string()
                    .chars()
                    .last()
                    .unwrap()
                    .to_digit(10)
                    .unwrap();
            }
        }
    }
    0
}

fn get_calibration_value(file_contents: &str) -> u32 {
    let mut calibration_value = 0;

    for line in file_contents.lines() {
        let first_digit = get_first_digit(line);
        let last_digit = get_last_digit(line);
        calibration_value += first_digit * 10 + last_digit;
    }

    calibration_value
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Should have been called with one argument");
    }
    let file_path = &args[1];
    let file_contents = fs::read_to_string(file_path).expect("Could not read file!");
    dbg!(file_path);

    let calibration_value = get_calibration_value(&file_contents);
    println!("{}", calibration_value)
}
