use regex::Regex;
use std::{error::Error, fs};

const REGEX_STRING: &str = "one|two|three|four|five|six|seven|eight|nine";

fn get_digits(line: &str) -> Vec<usize> {
    let re = Regex::new("\\d").unwrap();
    re.find_iter(line)
        .map(|m| m.as_str().parse::<usize>().unwrap())
        .collect()
}

fn get_first_digit_string(line: &str) -> &str {
    let re = Regex::new(&format!("\\d|{REGEX_STRING}")).unwrap();
    re.find(line).unwrap().as_str()
}

fn get_last_digit_string(line: &str) -> String {
    let re_str_rev: String = REGEX_STRING.chars().rev().collect();
    let re = Regex::new(&format!("\\d|{re_str_rev}")).unwrap();
    re.find(&line.chars().rev().collect::<String>())
        .unwrap()
        .as_str()
        .chars()
        .rev()
        .collect()
}

fn get_calibration_number(line: &str) -> usize {
    let digits = get_digits(line);
    let first = digits.first().unwrap();
    let last = digits.last().unwrap();
    format!("{first}{last}").parse::<usize>().unwrap()
}

fn parse_number_string(string: &str) -> usize {
    if let Ok(num) = string.parse::<usize>() {
        return num;
    }
    match string {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    }
}

fn get_stringified_calibration_number(line: &str) -> usize {
    let first = parse_number_string(get_first_digit_string(line));
    let last = parse_number_string(&get_last_digit_string(line));
    format!("{first}{last}").parse::<usize>().unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("data/day1")?;
    let lines = input.lines();

    let result: usize = lines.clone().map(get_calibration_number).sum();
    println!("Part 1: {result}");

    let result: usize = lines.map(get_stringified_calibration_number).sum();
    println!("Part 2: {result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _should_get_all_digits() {
        let ex0 = "1abc2";
        let mut digits = get_digits(ex0);
        assert_eq!(vec![1, 2], digits);

        let ex1 = "pqr3stu8vwx";
        digits = get_digits(ex1);
        assert_eq!(vec![3, 8], digits);

        let ex2 = "a1b2c3d4e5f";
        digits = get_digits(ex2);
        assert_eq!(vec![1, 2, 3, 4, 5], digits);

        let ex3 = "treb7uchet";
        digits = get_digits(ex3);
        assert_eq!(vec![7], digits);
    }

    #[test]
    fn _should_get_calibration_number() {
        let ex0 = "1abc2";
        let mut cal_num = get_calibration_number(ex0);
        assert_eq!(12, cal_num);

        let ex1 = "pqr3stu8vwx";
        cal_num = get_calibration_number(ex1);
        assert_eq!(38, cal_num);

        let ex2 = "a1b2c3d4e5f";
        cal_num = get_calibration_number(ex2);
        assert_eq!(15, cal_num);

        let ex3 = "treb7uchet";
        cal_num = get_calibration_number(ex3);
        assert_eq!(77, cal_num);
    }

    #[test]
    fn _should_parse_number_string() {
        let one = "one";
        assert_eq!(1, parse_number_string(one));

        let two = "two";
        assert_eq!(2, parse_number_string(two));

        let three = "three";
        assert_eq!(3, parse_number_string(three));

        let four = "four";
        assert_eq!(4, parse_number_string(four));

        let five = "five";
        assert_eq!(5, parse_number_string(five));

        let six = "six";
        assert_eq!(6, parse_number_string(six));

        let seven = "seven";
        assert_eq!(7, parse_number_string(seven));

        let eight = "eight";
        assert_eq!(8, parse_number_string(eight));

        let nine = "nine";
        assert_eq!(9, parse_number_string(nine));
    }

    #[test]
    fn _should_get_stringified_calibration_number() {
        let ex0 = "two1nine";
        assert_eq!(29, get_stringified_calibration_number(ex0));

        let ex1 = "eightwothree";
        assert_eq!(83, get_stringified_calibration_number(ex1));

        let ex2 = "abcone2threexyz";
        assert_eq!(13, get_stringified_calibration_number(ex2));

        let ex3 = "xtwone3four";
        assert_eq!(24, get_stringified_calibration_number(ex3));

        let ex4 = "4nineeightseven2";
        assert_eq!(42, get_stringified_calibration_number(ex4));

        let ex5 = "zoneight234";
        assert_eq!(14, get_stringified_calibration_number(ex5));

        let ex6 = "7pqrstsixteen";
        assert_eq!(76, get_stringified_calibration_number(ex6));
    }
}
