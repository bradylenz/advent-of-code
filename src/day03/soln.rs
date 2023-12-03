use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1() {
    if let Ok(lines) = read_lines("src/day03/input.txt") {
        let mut collected_lines: Vec<String> = Vec::new();

        for line in lines {
            if let Ok(ip) = line {
                collected_lines.push(ip);
            }
        }

        let mut curr_num: Vec<u32> = Vec::new();
        let mut curr_next_to_symbol = false;

        let mut sum = 0;

        for (i, line) in collected_lines.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    curr_num.push(c.to_digit(10).unwrap());
                    let (is_next_to_symbol, _) = is_adjacent_to_symbol(i, j, &collected_lines);
                    curr_next_to_symbol = curr_next_to_symbol || is_next_to_symbol;
                } else {
                    if curr_next_to_symbol {
                        let num = curr_num
                            .iter()
                            .enumerate()
                            .fold(0, |acc: u32, (idx, digit)| {
                                acc + 10_u32.pow((curr_num.len() - idx - 1).try_into().unwrap())
                                    * digit
                            });

                        sum += num;
                    }

                    curr_num.clear();
                    curr_next_to_symbol = false;
                }
            }

            if curr_next_to_symbol {
                let num = curr_num
                    .iter()
                    .enumerate()
                    .fold(0, |acc: u32, (idx, digit)| {
                        acc + 10_u32.pow((curr_num.len() - idx - 1).try_into().unwrap()) * digit
                    });

                sum += num;
            }

            curr_num.clear();
            curr_next_to_symbol = false;
        }

        println!("Part 1: {}", sum);
    }
}

fn is_adjacent_to_symbol(
    row: usize,
    col: usize,
    grid: &Vec<String>,
) -> (bool, Vec<(char, usize, usize)>) {
    let worklist = vec![
        (row.saturating_sub(1), col.saturating_sub(1)),
        (row.saturating_sub(1), col),
        (row.saturating_sub(1), col + 1),
        (row, col.saturating_sub(1)),
        (row, col + 1),
        (row + 1, col.saturating_sub(1)),
        (row + 1, col),
        (row + 1, col + 1),
    ];

    let mut adjacent_symbols = Vec::new();

    for (i, j) in worklist {
        if i >= grid.len() || j >= grid[i].len() {
            continue;
        }

        let c = grid[i].chars().nth(j).unwrap();
        if !c.is_ascii_digit() && c != '.' {
            adjacent_symbols.push((c, i, j));
        }
    }

    (adjacent_symbols.len() > 0, adjacent_symbols)
}

pub fn part2() {
    if let Ok(lines) = read_lines("src/day03/input.txt") {
        let mut collected_lines: Vec<String> = Vec::new();

        for line in lines {
            if let Ok(ip) = line {
                collected_lines.push(ip);
            }
        }

        let mut curr_num: Vec<u32> = Vec::new();
        let mut curr_next_to_symbol = false;
        let mut curr_adj_symbols = Vec::new();

        let mut sum = 0;
        let mut results: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

        for (i, line) in collected_lines.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    curr_num.push(c.to_digit(10).unwrap());

                    let (is_next_to_symbol, symbols) =
                        is_adjacent_to_symbol(i, j, &collected_lines);
                    curr_adj_symbols.extend(symbols);
                    curr_next_to_symbol = curr_next_to_symbol || is_next_to_symbol;
                } else {
                    if curr_next_to_symbol {
                        let num = curr_num
                            .iter()
                            .enumerate()
                            .fold(0, |acc: u32, (idx, digit)| {
                                acc + 10_u32.pow((curr_num.len() - idx - 1).try_into().unwrap())
                                    * digit
                            });

                        for entry in curr_adj_symbols.iter() {
                            if entry.0 == '*' {
                                let key = (entry.1, entry.2);

                                if results.contains_key(&key) {
                                    results.get_mut(&key).unwrap().push(num);
                                } else {
                                    results.insert(key, vec![num]);
                                }

                                break;
                            }
                        }
                    }

                    curr_adj_symbols.clear();
                    curr_num.clear();
                    curr_next_to_symbol = false;
                }
            }

            if curr_next_to_symbol {
                let num = curr_num
                    .iter()
                    .enumerate()
                    .fold(0, |acc: u32, (idx, digit)| {
                        acc + 10_u32.pow((curr_num.len() - idx - 1).try_into().unwrap()) * digit
                    });

                for entry in curr_adj_symbols.iter() {
                    if entry.0 == '*' {
                        let key = (entry.1, entry.2);

                        if results.contains_key(&key) {
                            results.get_mut(&key).unwrap().push(num);
                        } else {
                            results.insert(key, vec![num]);
                        }

                        break;
                    }
                }
            }

            curr_adj_symbols.clear();
            curr_num.clear();
            curr_next_to_symbol = false;
        }

        for (_, nums) in results {
            if nums.len() > 1 {
                sum += nums.iter().fold(1, |acc, num| acc * num);
            }
        }

        println!("Part 2: {}", sum);
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
