use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1() {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    if let Ok(lines) = read_lines("src/day02/input.txt") {
        let mut sum = 0;

        for line in lines {
            if let Ok(ip) = line {
                let game_info = ip.split(':').collect::<Vec<&str>>();

                let game_id = game_info[0].split(' ').collect::<Vec<&str>>()[1]
                    .parse::<i32>()
                    .unwrap();
                let mut is_valid_round = true;

                for round_info in game_info[1].split(';') {
                    for cube_info in round_info.split(',') {
                        let count = cube_info.trim().split(' ').collect::<Vec<&str>>()[0]
                            .parse::<i32>()
                            .unwrap();
                        let color = cube_info.trim().split(' ').collect::<Vec<&str>>()[1];

                        let is_count_valid = match color {
                            "red" => count <= max_red,
                            "green" => count <= max_green,
                            "blue" => count <= max_blue,
                            _ => true,
                        };

                        is_valid_round = is_valid_round && is_count_valid;
                        if !is_valid_round {
                            continue;
                        }
                    }
                }

                if is_valid_round {
                    sum += game_id;
                }
            }
        }

        println!("Part 1: {}", sum);
    }
}

pub fn part2() {
    if let Ok(lines) = read_lines("src/day02/input.txt") {
        let mut sum = 0;

        for line in lines {
            if let Ok(ip) = line {
                let game_info = ip.split(':').collect::<Vec<&str>>();

                let mut max_red = 0;
                let mut max_green = 0;
                let mut max_blue = 0;

                for round_info in game_info[1].split(';') {
                    for cube_info in round_info.split(',') {
                        let count = cube_info.trim().split(' ').collect::<Vec<&str>>()[0]
                            .parse::<i32>()
                            .unwrap();
                        let color = cube_info.trim().split(' ').collect::<Vec<&str>>()[1];

                        match color {
                            "red" => max_red = std::cmp::max(max_red, count),
                            "green" => max_green = std::cmp::max(max_green, count),
                            "blue" => max_blue = std::cmp::max(max_blue, count),
                            _ => (),
                        };
                    }
                }

                sum += max_red * max_green * max_blue;
            }
        }

        println!("Part 1: {}", sum);
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
