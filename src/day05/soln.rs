use std::fs::read_to_string;

pub fn part1() {
    // let lines = read_lines("src/day05/input-small.txt");
    let lines = read_lines("src/day05/input.txt");

    let mut source_nums = parse_seed_nums(&lines.get(0).unwrap());

    let map_lines = &lines[2..];
    let maps = get_maps(map_lines);

    for source_num in source_nums.iter_mut() {
        for map in maps.iter() {
            *source_num = map.map_num(source_num);
        }
    }

    println!("Part 1: {}", source_nums.iter().min().unwrap());
}

pub fn part2() {
    // let lines = read_lines("src/day05/input-small.txt");
    let lines = read_lines("src/day05/input.txt");

    let source_ranges = &mut parse_seed_ranges(&lines.get(0).unwrap());

    let map_lines = &lines[2..];
    let maps = get_maps(map_lines);

    for map in maps.iter() {
        map.map_ranges(source_ranges);
    }

    let smallest_mapping = source_ranges.iter().map(|r| r.0).min().unwrap();

    println!("Part 2: {}", smallest_mapping);
}

fn get_maps(map_lines: &[String]) -> Vec<Map> {
    let mut maps: Vec<Map> = Vec::new();
    let mut mapping_start = 0;

    for (i, line) in map_lines.iter().enumerate() {
        let end_mapping = line.is_empty() || i == map_lines.len() - 1;
        if end_mapping {
            let mapping_lines = &map_lines[mapping_start + 1..i + 1];

            let mut map = Map::new();
            map.populate_from_str(mapping_lines);
            maps.push(map);

            mapping_start = i + 1;
        }
    }

    return maps;
}

fn parse_seed_ranges(seed_str: &str) -> Vec<(i64, i64)> {
    parse_seed_nums(seed_str)
        .chunks(2)
        .map(|range| (range[0], range[1]))
        .collect::<Vec<(i64, i64)>>()
}

fn parse_seed_nums(seed_str: &str) -> Vec<i64> {
    seed_str
        .splitn(2, ':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

struct Mapping {
    source_start: i64,
    dest_start: i64,
    len: i64,
}

impl Mapping {
    pub fn map_num(&self, source_num: &i64) -> (bool, i64) {
        if source_num >= &self.source_start && source_num < &(self.source_start + self.len) {
            return (true, self.dest_start + (source_num - self.source_start));
        }

        return (false, *source_num);
    }

    pub fn map_ranges(
        &self,
        source_ranges: &Vec<(i64, i64)>,
    ) -> (Vec<(i64, i64)>, Vec<(i64, i64)>) {
        let mut ranges_to_map = source_ranges.clone();
        let mut mapped_ranges: Vec<(i64, i64)> = Vec::new();
        let mut non_mapped_ranges: Vec<(i64, i64)> = Vec::new();

        while !ranges_to_map.is_empty() {
            let curr_range = ranges_to_map.pop().unwrap();

            let (start_success, mapped_start) = self.map_num(&curr_range.0);
            let (end_success, _mapped_end) = self.map_num(&(curr_range.0 + curr_range.1 - 1));

            if start_success {
                let mapped_len =
                    std::cmp::min(self.source_start + self.len, curr_range.0 + curr_range.1)
                        - curr_range.0;
                mapped_ranges.push((mapped_start, mapped_len));

                let remainder = curr_range.1 - mapped_len;
                if remainder > 0 {
                    ranges_to_map.push((curr_range.0 + mapped_len, remainder));
                }
            } else if end_success {
                let mapped_len = curr_range.0 + curr_range.1 - self.source_start;
                mapped_ranges.push((self.dest_start, mapped_len));

                let remainder = curr_range.1 - mapped_len;
                if remainder > 0 {
                    ranges_to_map.push((curr_range.0, remainder));
                }
            } else {
                non_mapped_ranges.push(curr_range);
            }
        }

        (mapped_ranges.clone(), non_mapped_ranges.clone())
    }
}

struct Map {
    mappings: Vec<Mapping>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            mappings: Vec::new(),
        }
    }

    pub fn populate_from_str(&mut self, lines: &[String]) -> () {
        let mappings = lines
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s1| {
                s1.split(' ')
                    .map(|num| num.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            })
            .map(|nums| Mapping {
                source_start: *nums.get(1).unwrap(),
                dest_start: *nums.get(0).unwrap(),
                len: *nums.get(2).unwrap(),
            });

        self.mappings.extend(mappings);
    }

    pub fn map_num(&self, source_num: &i64) -> i64 {
        for mapping in self.mappings.iter() {
            let (success, mapped_num) = mapping.map_num(source_num);
            if success {
                return mapped_num;
            }
        }

        *source_num
    }

    pub fn map_ranges(&self, source_ranges: &mut Vec<(i64, i64)>) -> () {
        let mut ranges_to_map = source_ranges.clone();
        let mut mapped_ranges: Vec<(i64, i64)> = Vec::new();

        for mapping in self.mappings.iter() {
            let result = mapping.map_ranges(&ranges_to_map);
            mapped_ranges.extend(result.0);

            ranges_to_map.clear();
            ranges_to_map.extend(result.1);
        }

        source_ranges.clear();
        source_ranges.extend(mapped_ranges);
        source_ranges.extend(ranges_to_map);
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
