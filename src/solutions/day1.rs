use std::collections::HashMap;
use crate::file_reader::read_file_lines;

fn iterate_over_chars(line : &String) -> (Option<u32>, Option<u32>, usize, usize) {
    let mut first_digit : Option<u32> = None;
    let mut last_digit : Option<u32> = None;

    let mut first_index: usize = line.len() - 1;
    let mut last_index = 0;

    for c in line.chars().enumerate() {
        if c.1.is_ascii_digit() {
            if first_digit.is_none() {
                first_index = c.0;
                first_digit = Some(c.1.to_digit(10).unwrap());
            }
            last_digit = Some(c.1.to_digit(10).unwrap());
            if last_digit.is_some() {
                last_index = c.0;
            }
        }
    }

    (first_digit, last_digit, first_index, last_index)
}

pub fn solve_a() {
    let mut sum : u32 = 0;

    for line in read_file_lines("input_data/day1/day1b.in") {
        let ( first_digit, last_digit, _, _)
            = iterate_over_chars(&line);

        if first_digit.is_some() && last_digit.is_some() {
            sum += first_digit.unwrap() * 10 + last_digit.unwrap();
        }
    }

    println!("{}", sum)
}

pub fn solve_b(){
    let number_dict: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut sum : u32 = 0;

    for line in read_file_lines("input_data/day1/day1b.in") {
        let (mut first_digit, mut last_digit, first_index, last_index)
            = iterate_over_chars(&line);

        if first_index > 2 {
            let possible_extension = number_dict.keys().map( |str|{
                (line[0..first_index].find(*str).unwrap_or(usize::MAX), *str)
            }).min().unwrap();

            if possible_extension.0 != usize::MAX {
                first_digit = Some(number_dict[possible_extension.1])
            }
        }

        if last_index < line.len() - 2  {
            let possible_extension = number_dict.keys().map( |str|{
                (line[last_index..line.len()].rfind(*str), *str)
            }).max().unwrap();

            if possible_extension.0.is_some() {
                last_digit = Some(number_dict[possible_extension.1])
            }
        }

        if first_digit.is_some() && last_digit.is_some() {
            sum += first_digit.unwrap() * 10 + last_digit.unwrap();
        }
    }

    println!("{}", sum)
}