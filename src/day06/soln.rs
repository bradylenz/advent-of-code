use std::fs::read_to_string;

pub fn part1() {
    let lines = read_lines("src/day06/input.txt");

    let race_times = lines
        .get(0)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    let distance_records = lines
        .get(1)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    let mut product = 1;

    for (race_time, record_distance) in std::iter::zip(race_times.iter(), distance_records.iter()) {
        let upper =
            ((race_time + (race_time.powi(2) - 4.0 * record_distance).sqrt()) / 2.0).floor() as u64;
        let lower =
            ((race_time - (race_time.powi(2) - 4.0 * record_distance).sqrt()) / 2.0).floor() as u64;

        product *= upper - lower;
    }

    println!("Part 1: {}", product);
}

pub fn part2() {
    let lines = read_lines("src/day06/input.txt");

    let race_time = lines
        .get(0)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<String>()
        .parse::<f64>()
        .unwrap();

    let record_distance = lines
        .get(1)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<String>()
        .parse::<f64>()
        .unwrap();

    let upper =
        ((race_time + (race_time.powi(2) - 4.0 * record_distance).sqrt()) / 2.0).floor() as u64;
    let lower =
        ((race_time - (race_time.powi(2) - 4.0 * record_distance).sqrt()) / 2.0).floor() as u64;

    println!("Part 2: {}", upper - lower);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
