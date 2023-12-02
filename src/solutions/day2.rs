use std::cmp::max;
use crate::file_reader;

use std::collections::HashMap;

fn game_data_parser(game_data: &str) -> (i32, &str){
    let data = game_data.split(":").collect::<Vec<&str>>();
    let game_data = data[0].split_whitespace().collect::<Vec<&str>>();
    let game_id = game_data[1].parse::<i32>().unwrap();

    (game_id, data[1])
}

fn color_parser(color_str : &str) -> (&str, i32) {
    let split_color_data =  color_str.trim().split_whitespace().collect::<Vec<&str>>();
    let count = split_color_data[0].parse::<i32>().unwrap_or(-1);
    let color = split_color_data[1];

    (color, count)
}

pub fn solve_a() {
    let mut id_sum = 0;
    let color_max: HashMap<&str, i32> = HashMap::from(
        [
            ("red", 12),
            ("green", 13),
            ("blue", 14)
        ]
    );

    'iterate_lines: for line in file_reader::read_file_lines("input_data/day2/day2.in") {
        let (game_id, game_data) = game_data_parser(line.as_str());

        for color_data in game_data.split([';', ',']) {
            let (color, count) = color_parser(color_data);

            if count > color_max[color] {
                continue 'iterate_lines
            }
        }

        id_sum += game_id
    }

    println!("{}", id_sum)
}

pub fn solve_b() {
    let mut power_sum = 0;

    for line in file_reader::read_file_lines("input_data/day2/day2.in") {
        let (_, game_data) = game_data_parser(line.as_str());

        let mut color_count: HashMap<&str, i32> = HashMap::from([
                ("red", i32::MIN),
                ("green", i32::MIN),
                ("blue", i32::MIN)]);

        for color_data in game_data.split([';', ',']) {
            let (color, count) = color_parser(color_data);
            _ = color_count.insert(color,max(color_count[color], count))
        }

        power_sum += color_count.values().fold(1, |sum, val| val*sum);
    }

    println!("{}", power_sum)
}