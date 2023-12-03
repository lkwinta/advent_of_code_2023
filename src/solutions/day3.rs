use std::collections::HashMap;
use crate::file_reader;

fn check_neighbours(lines : &Vec<String>, line_index : i32, char_index : i32) -> bool {
    for i in -1..2 {
        for j in -1..2 {
            if !(i == 0 && j == 0) {
                let new_line_index = line_index + i;
                let new_char_index = char_index + j;

                if  new_line_index >= 0 && (new_line_index as usize) < lines.len() &&
                    new_char_index >= 0 && (new_char_index as usize) < lines[0].len(){
                    let c = lines[new_line_index as usize].chars().nth(new_char_index as usize).unwrap();
                    if c != '.' && !c.is_ascii_digit() {
                        return true
                    }
                }
            }
        }
    }

    false
}

fn update_sum(current_has_neighbour : &mut bool, current_number : &mut String, sum : &mut i32){
    if *current_has_neighbour {
        *sum += (*current_number).parse::<i32>().unwrap();
    }

    *current_number = String::from("");
    *current_has_neighbour = false;
}

pub fn solve_a() {
    let lines = file_reader::read_file_lines("input_data/day3/day3.in");
    let mut current_number : String = String::from("");
    let mut sum = 0;
    let mut current_has_neighbour = false;

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                if check_neighbours(&lines, i as i32, j as i32) {
                    current_has_neighbour = true;
                }
                current_number.push(c);
            } else {
                update_sum(&mut current_has_neighbour, &mut current_number, &mut sum);
            }
        }

        update_sum(&mut current_has_neighbour, &mut current_number, &mut sum);
    }

    println!("{}", sum);
}

fn count_adjacent(numbers_map : & HashMap<(usize, usize), i32>, pos_i: &usize, pos_j: &usize) -> (i32, i32) {
    let mut counter = 0;
    let mut multiplication = 1;

    for i in -1..2 {
        for j in -1..2 {
            if !(i == 0 && j == 0) {
                let new_pos_i = *pos_i as i32 + i;
                let new_pos_j = *pos_j as i32 + j;

                if  new_pos_i >= 0 && new_pos_j >= 0 {
                    if numbers_map.contains_key(&(new_pos_i as usize, new_pos_j as usize)) {
                        if (j == 0 || j == 1) && new_pos_j - 1 >= 0 && numbers_map.contains_key(&(new_pos_i as usize, (new_pos_j - 1) as usize)) {
                            continue
                        }

                        let num = numbers_map.get(&(new_pos_i as usize, new_pos_j as usize)).unwrap();

                        multiplication *= num;
                        counter += 1
                    }
                }
            }
        }
    }

    (counter, multiplication)
}

pub fn solve_b() {
    let lines = file_reader::read_file_lines("input_data/day3/day3.in");
    let mut current_number : String = String::from("");

    let mut numbers_map : HashMap<(usize, usize), i32> = HashMap::new();
    let mut visited_list: Vec<(usize, usize)> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                current_number.push(c);
                visited_list.push((i, j));
            } else {

                if current_number == "" {
                    continue
                }

                let number = current_number.parse::<i32>().unwrap();

                for (pos_i, pos_j) in &visited_list {
                    numbers_map.insert((*pos_i, *pos_j), number);

                }
                visited_list.clear();
                current_number = String::from("");
            }
        }

        if current_number == "" {
            continue
        }

        let number = current_number.parse::<i32>().unwrap();

        for (pos_i, pos_j) in &visited_list {
            numbers_map.insert((*pos_i, *pos_j), number);

        }
        visited_list.clear();
        current_number = String::from("");
    }

    let mut sum = 0;

    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '*' {
                let (count, multiplication) = count_adjacent(&numbers_map, &i, &j);

                if count == 2 {
                    sum += multiplication
                }
            }
        }
    }

    println!("{}", sum);
}