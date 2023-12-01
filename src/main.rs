use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("{}", env::current_dir().unwrap().to_str().unwrap());
    let input_a_file = File::open("input_data/day1a.in").unwrap();
    let input_a_reader = BufReader::new(input_a_file);
    let mut sum : u32 = 0;

    for line in  input_a_reader.lines() {
        let mut first_digit : Option<u32> = None;
        let mut last_digit : Option<u32> = None;

        line.unwrap().chars().for_each(|c|{
            if ('0'..'9').contains(&c) {
                if first_digit.is_none() { first_digit = Some(c.to_digit(10).unwrap()) }
                last_digit = Some(c.to_digit(10).unwrap())
            }
        });

        if first_digit.is_some() && last_digit.is_some() {
            println!("{}", first_digit.unwrap() * 10 + last_digit.unwrap());
            sum += first_digit.unwrap() * 10 + last_digit.unwrap();
        }
    }

    println!("{}", sum)
}
