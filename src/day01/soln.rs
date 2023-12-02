use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1() {
    if let Ok(lines) = read_lines("src/day01/input.txt") {
        let mut sum = 0;

        for line in lines {
            if let Ok(ip) = line {
                let mut calibration_value = String::new();
                for c in ip.chars() {
                    if c.is_digit(10) {
                        calibration_value.push(c);
                        break;
                    }
                }

                for c in ip.chars().rev() {
                    if c.is_digit(10) {
                        calibration_value.push(c);
                        break;
                    }
                }

                let calibration_num = calibration_value.parse::<i32>().unwrap();
                sum += calibration_num;
            }
        }

        println!("Part 1: {}", sum);
    }
}

pub fn part2() {
    let digit_list = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    if let Ok(lines) = read_lines("src/day01/input.txt") {
        let mut sum = 0;

        for line in lines {
            if let Ok(ip) = line {
                let mut min_idx = usize::MAX;
                let mut min_digit = "0";
                let mut max_idx = usize::MIN;
                let mut max_digit = "0";

                for digit in digit_list.iter() {
                    match (ip.find(digit), ip.rfind(digit)) {
                        (Some(idx_1), Some(idx_2)) => {
                            if idx_1 <= min_idx {
                                min_idx = idx_1;
                                min_digit = digit;
                            }
                            if idx_2 >= max_idx {
                                max_idx = idx_2;
                                max_digit = digit;
                            }
                        }
                        _ => (),
                    }
                }

                let mut calibration_value = String::new();
                calibration_value.push(map_digit(min_digit));
                calibration_value.push(map_digit(max_digit));
                let calibration_num = calibration_value.parse::<i32>().unwrap();

                sum += calibration_num;
            }
        }

        println!("Part 2: {}", sum);
    }
}

fn map_digit(digit: &str) -> char {
    match digit {
        "1" => '1',
        "2" => '2',
        "3" => '3',
        "4" => '4',
        "5" => '5',
        "6" => '6',
        "7" => '7',
        "8" => '8',
        "9" => '9',
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => '0',
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
