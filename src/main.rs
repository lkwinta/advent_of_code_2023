use std::arch::x86_64::_mm_undefined_si128;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


fn solve_a() {
    let input_a_file = File::open("input_data/day1a.in").unwrap();
    let input_a_reader = BufReader::new(input_a_file);

    let mut sum : u32 = 0;

    for line in  input_a_reader.lines() {
        let line = line.unwrap();

        let mut first_digit : Option<u32> = None;
        let mut last_digit : Option<u32> = None;

        line.chars().for_each(|c|{
            if c.is_ascii_digit() {
                if first_digit.is_none() { first_digit = Some(c.to_digit(10).unwrap()) }
                last_digit = Some(c.to_digit(10).unwrap())
            }
        });

        if first_digit.is_some() && last_digit.is_some() {
            sum += first_digit.unwrap() * 10 + last_digit.unwrap();
        }
    }

    println!("{}", sum)
}

fn solve_b(){
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


    let input_b_file = File::open("input_data/day1b.in").unwrap();
    let input_b_reader = BufReader::new(input_b_file);

    let mut sum : u32 = 0;

    for line in  input_b_reader.lines() {
        let line = line.unwrap();

        let mut first_digit : Option<u32> = None;
        let mut last_digit : Option<u32> = None;

        let mut first_index: usize = line.len() - 1;
        let mut last_index = 0;

        for c in line.chars().enumerate() {
            if c.1.is_ascii_digit() {
                if first_digit.is_none() {
                    first_index = c.0;
                    first_digit = Some(c.1.to_digit(10).unwrap())
                }
                last_digit = Some(c.1.to_digit(10).unwrap());
                if last_digit.is_some() {
                    last_index = c.0
                }
            }
        }

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

fn main() {
    solve_a();
    solve_b();
}
