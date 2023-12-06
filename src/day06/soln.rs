use std::fs::read_to_string;

pub fn part1() {
    // let lines = read_lines("src/day06/input-small.txt");
    let lines = read_lines("src/day06/input.txt");

    let race_times = lines
        .get(0)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let distance_records = lines
        .get(1)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut product = 1;

    for (race_time, record_distance) in std::iter::zip(race_times.iter(), distance_records.iter()) {
        let mut ways = 0;

        for i in 0..*race_time {
            let dist = i * (race_time - i);
            if dist > *record_distance {
                ways += 1;
            }
        }

        product *= ways;
    }

    println!("Part 1: {}", product);
}

pub fn part2() {
    // let lines = read_lines("src/day06/input-small.txt");
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
        .parse::<u64>()
        .unwrap();

    let distance_record = lines
        .get(1)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let mut ways = 0;
    for i in 0..race_time {
        let dist = i * (race_time - i);
        if dist > distance_record {
            ways += 1;
        }
    }

    println!("Part 2: {}", ways);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
