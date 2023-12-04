use crate::file_reader;
use std::collections::{HashMap, HashSet};

fn card_data_parser(card_data_str: &str) -> (i32, &str){
    let data = card_data_str.split(":").collect::<Vec<&str>>();
    let card_data = data[0].split_whitespace().collect::<Vec<&str>>();
    let card_id = card_data[1].parse::<i32>().unwrap();

    (card_id, data[1])
}

fn card_values_parser(card_values_str : &str) -> (Vec<i32>, Vec<i32>) {
    let values =  card_values_str.trim().split('|').collect::<Vec<&str>>();
    let wining_values = values[0].split_whitespace().map(|str| str.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let your_values = values[1].split_whitespace().map(|str| str.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    (wining_values, your_values)
}


pub fn solve_a(){
    let mut score_sum = 0;

    for line in file_reader::read_file_lines("input_data/day4/day4.in"){
        let (_, card_data) = card_data_parser(line.as_str());
        let (wining_values, your_values) = card_values_parser(card_data);
        let wining_values_set : HashSet<i32> = HashSet::from_iter(wining_values);

        let n = your_values.iter().filter(|i|wining_values_set.contains(*i)).count() as u32;
        score_sum += match n { 0 => 0, _ => i32::pow(2, n - 1) };
    }

    println!("{}", score_sum);
}

pub fn solve_b(){
    let mut cards_map : HashMap<i32, i32> = HashMap::new();

    for line in  file_reader::read_file_lines("input_data/day4/day4.in") {
        let (card_id, card_data) = card_data_parser(line.as_str());
        let (wining_values, your_values) = card_values_parser(card_data);

        let wining_values_set : HashSet<i32> = HashSet::from_iter(wining_values);
        let card_score = your_values.iter().filter(|i| wining_values_set.contains(*i)).count() as i32;

        let current_card_count = cards_map.get(&card_id).unwrap_or_else(|| &0) + 1;
        cards_map.insert(card_id, current_card_count);

        for i in (card_id + 1)..(card_id + card_score + 1) {
            cards_map.insert(i, cards_map.get(&i).unwrap_or_else(|| &0) + current_card_count);
        }
    }

    let number_of_cards : i32 = cards_map.values().sum();
    println!("{}", number_of_cards);
}