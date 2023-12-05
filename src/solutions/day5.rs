use std::collections::HashSet;
use std::ops::Range;
use crate::file_reader;

fn seeds_parser(str: &String) -> Vec<i64> {
    return str.split(':')
        .nth(1).unwrap()
        .split(' ')
        .filter(|seed| !seed.is_empty())
        .map(|seed| seed.trim().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
}

fn range_parser(str: &String) -> (i64, i64, i64) {
    let range = str
        .split(' ')
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    (range[0], range[1], range[2])
}

pub fn solve_a(){
    let lines = file_reader::read_file_lines("input_data/day5/day5.in");
    let sections = lines.split(|line| line == "").collect::<Vec<_>>();

    let seeds = seeds_parser(&sections[0][0]);

    let mut previous_stage: HashSet<i64> = HashSet::from_iter(seeds);
    let mut current_stage: HashSet<i64> = HashSet::new();

    for i in 1..sections.len(){
        for j in 1..sections[i].len(){
            let (destination, source, length) = range_parser(&sections[i][j]);

            for src_num in previous_stage.clone() {
                if (source..source+length).contains(&src_num) {
                    current_stage.insert(destination + (src_num - source));
                    previous_stage.remove(&src_num);
                }
            }
        }

        for src_num in &previous_stage {
            current_stage.insert(*src_num);
        }

        previous_stage = current_stage;
        current_stage = HashSet::new();
    }

    let min_location = previous_stage.iter().min().unwrap();
    println!("{}", min_location)
}

pub fn solve_b(){
    let lines = file_reader::read_file_lines("input_data/day5/day5.in");
    let sections = lines.split(|line| line == "").collect::<Vec<_>>();

    let seeds = seeds_parser(&sections[0][0]);

    let mut previous_stage: HashSet<Range<i64>> = HashSet::new();
    let mut current_stage: HashSet<Range<i64>> = HashSet::new();

    for i in (0..seeds.len()).step_by(2) {
        previous_stage.insert(seeds[i]..seeds[i]+seeds[i+1]);
    }

    for i in 1..sections.len(){
        for j in 1..sections[i].len(){
            let (destination, source, length) = range_parser(&sections[i][j]);
            let range_source = source..source+length;
            let diff = destination - source;

            for previous_range in previous_stage.clone() {
                if previous_range.end <= range_source.start || previous_range.start >= range_source.end {
                    continue;
                }
                else if previous_range.start >= range_source.start && previous_range.end <= range_source.end {
                    current_stage.insert(Range{start: previous_range.start + diff, end: previous_range.end + diff });
                }
                else if previous_range.start < range_source.start && previous_range.end > range_source.end {
                    current_stage.insert(range_source.start + diff..range_source.end + diff);
                    previous_stage.insert(previous_range.start..range_source.start);
                    previous_stage.insert(range_source.end..previous_range.end);
                }
                else if previous_range.start < range_source.start && previous_range.end <= range_source.end {
                    current_stage.insert(range_source.start + diff..previous_range.end + diff);
                    previous_stage.insert(previous_range.start..range_source.start);
                }
                else if previous_range.start >= range_source.start && previous_range.end > range_source.end {
                    current_stage.insert(previous_range.start + diff..range_source.end + diff);
                    previous_stage.insert(range_source.end..previous_range.end);
                }
                previous_stage.remove(&previous_range);
            }
        }

        for left_range in previous_stage {
            current_stage.insert(left_range);
        }

        previous_stage = current_stage;
        current_stage = HashSet::new();
    }

    let min = previous_stage.iter().map(|range| range.start).min().unwrap();

    println!("{}", min)
}