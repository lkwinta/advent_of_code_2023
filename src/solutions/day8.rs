use crate::file_reader;
use std::collections::HashMap;

pub fn solve_a() {
    let lines = file_reader::read_file_lines("input_data/day8/day8.in");
    let direction = lines[0].chars().collect::<Vec<char>>();

    let mut graph: HashMap<&str, [String; 2]> = HashMap::new();

    for line in lines.iter().skip(2) {
        let mut line_split = line.split(" = ");
        let node = line_split.nth(0).unwrap();
        let targets = line_split.nth(0).unwrap().replace("(", "").replace(")", "");
        let target_nodes = targets.split(", ").collect::<Vec<&str>>();

        graph.insert( node, [String::from(target_nodes[0]), String::from(target_nodes[1])]);
    }

    let mut current = "AAA";
    let mut i = 0;
    let mut counter = 0;

    while current != "ZZZ" {
        if direction[i] == 'L' {
            current = graph.get(current).unwrap()[0].as_str();
        } else {
            current = graph.get(current).unwrap()[1].as_str();
        }

        counter += 1;
        i = (i + 1) % direction.len();
    }

    println!("{}", counter);
}

fn nwd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let c = a % b;
        a = b;
        b = c;
    }

    a
}

pub fn solve_b() {
    let lines = file_reader::read_file_lines("input_data/day8/day8.in");
    let direction = lines[0].chars().collect::<Vec<char>>();

    let mut graph: HashMap<&str, [String; 2]> = HashMap::new();

    for line in lines.iter().skip(2) {
        let mut line_split = line.split(" = ");
        let node = line_split.nth(0).unwrap();
        let targets = line_split.nth(0).unwrap().replace("(", "").replace(")", "");
        let target_nodes = targets.split(", ").collect::<Vec<&str>>();

        graph.insert( node, [String::from(target_nodes[0]), String::from(target_nodes[1])]);
    }

    let mut current: Vec<&str> = Vec::new();
    for key in graph.keys() {
        if (*key).ends_with('A'){
            current.push(*key);
        }
    }
    let mut current_counter: Vec<usize> = vec![0; current.len()];

    for i in 0..current.len() {
        let mut counter = 0;
        let mut direction_i = 0;

        while !current[i].ends_with('Z') {
            if direction[direction_i] == 'L' {
                current[i] = graph.get(current[i]).unwrap()[0].as_str();
            } else {
                current[i] = graph.get(current[i]).unwrap()[1].as_str();
            }

            counter += 1;
            direction_i = (direction_i + 1) % direction.len();
        }

        current_counter[i] = counter;
    }

    let mut nww: usize = current_counter[0];

    for i in 1..current_counter.len(){
        nww = nww*current_counter[i]/nwd(nww, current_counter[i])
    }

    println!("{:?}", nww);
}