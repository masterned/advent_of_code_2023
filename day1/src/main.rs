use regex::Regex;
use std::{error::Error, fs};

fn get_digits(line: &str) -> Vec<usize> {
    let re = Regex::new("\\d").unwrap();
    re.find_iter(line)
        .map(|m| m.as_str().parse::<usize>().unwrap())
        .collect()
}

fn get_calibration_number(line: &str) -> usize {
    let digits = get_digits(line);
    let first = digits.first().unwrap();
    let last = digits.last().unwrap();
    format!("{first}{last}").parse::<usize>().unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("data/day1")?;
    let lines = input.lines();
    let result: usize = lines.map(get_calibration_number).sum();
    println!("Part 1: {result}");
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
}
