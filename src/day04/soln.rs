use std::fs::read_to_string;

pub fn part1() {
    // let mut lines = read_lines("src/day04/p1-small-input.txt");
    let lines = read_lines("src/day04/input.txt");

    let mut sum = 0;

    for line in lines.iter() {
        let (winning_nums, nums) = parse_nums(line);

        let num_matches = get_matches(&winning_nums, &nums);

        if num_matches > 0 {
            sum += 2_i32.pow(num_matches - 1);
        }
    }

    println!("Part 1: {}", sum);
}

pub fn part2() {
    // let mut lines = read_lines("src/day04/p1-small-input.txt");
    let lines = read_lines("src/day04/input.txt");

    let mut total_scratchcards = 0;

    let original_scratchcards = lines
        .iter()
        .enumerate()
        .map(|(i, s)| (i, parse_nums(s)))
        .collect::<Vec<(usize, (Vec<i32>, Vec<i32>))>>();

    let mut worklist = original_scratchcards.clone();

    while !worklist.is_empty() {
        total_scratchcards += 1;

        let (idx, (winning_nums, nums)) = worklist.pop().unwrap();

        let num_matches: usize = get_matches(&winning_nums, &nums).try_into().unwrap();
        if num_matches > 0 {
            let next_idx = idx + 1;
            let last_idx = std::cmp::min(original_scratchcards.len(), idx + num_matches + 1);

            worklist.extend_from_slice(&original_scratchcards[next_idx..last_idx]);
        }
    }

    println!("Part 2: {}", total_scratchcards);
}

fn get_matches(winning_nums: &Vec<i32>, nums: &Vec<i32>) -> u32 {
    nums.iter()
        .filter(|n| winning_nums.contains(n))
        .count()
        .try_into()
        .unwrap()
}

fn parse_nums(card_str: &str) -> (Vec<i32>, Vec<i32>) {
    let parsed_nums = card_str
        .splitn(2, ':')
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .splitn(2, '|')
        .map(|nums| {
            nums.split(' ')
                .filter(|num| !num.is_empty())
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let winning_nums = parsed_nums.get(0).unwrap();
    let nums = parsed_nums.get(1).unwrap();

    return (winning_nums.to_vec(), nums.to_vec());
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
