use crate::file_reader;
use std::collections::HashMap;

#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Debug)]
#[repr(u8)]
pub enum Pipe {
    UpDown = b'|',
    LeftRight = b'-',
    UpLeft = b'J',
    UpRight = b'L',
    DownLeft = b'7',
    DownRight = b'F',
    StartPipe = b'S',

    NoPath = b'.'
}
impl Into<char> for Pipe {
    fn into(self) -> char {
        self as u8 as char
    }
}

impl Pipe {
    pub fn from_u8(num: u8) -> Pipe {
        match num as char {
            '|' => Pipe::UpDown,
            '-' => Pipe::LeftRight,
            'J' => Pipe::UpLeft,
            'L' => Pipe::UpRight,
            '7' => Pipe::DownLeft,
            'F' => Pipe::DownRight,
            'S' => Pipe::StartPipe,
            '.' => Pipe::NoPath,

            _ => panic!("Illegal character!")
        }
    }
}

impl Into<Pipe> for char {
    fn into(self) -> Pipe {
        Pipe::from_u8(self as u8)
    }
}

fn parse_map(lines: Vec<String>) -> Vec<Vec<Pipe>>{
    lines.iter()
        .map(|str| str.chars().map(|c| c.into()).collect())
        .collect::<Vec<Vec<Pipe>>>()
}

fn find_start_point(map: &Vec<Vec<Pipe>>) -> (usize, usize) {
    let mut x_start: usize = usize::MAX;
    let mut y_start: usize = usize::MAX;

    'outer: for (y, line) in map.iter().enumerate() {
        for (x, pipe) in line.iter().enumerate() {
            if *pipe == Pipe::StartPipe {
                x_start = x;
                y_start = y;
                break 'outer
            }
        }
    }

    (y_start, x_start)
}

fn traverse_current(current: (usize, usize), prev: (usize, usize), current_pipe: Pipe) -> (usize, usize){
    match current_pipe {
        Pipe::UpRight => {
            if (current.0 - 1, current.1) != prev { (current.0 - 1, current.1) }
            else { (current.0, current.1 + 1) }
        },
        Pipe::UpDown => {
            if (current.0 - 1, current.1) != prev { (current.0 - 1, current.1) }
            else { (current.0 + 1, current.1) }
        },
        Pipe::UpLeft => {
            if (current.0 - 1, current.1) != prev { (current.0 - 1, current.1) }
            else { (current.0, current.1 - 1) }
        },
        Pipe::LeftRight => {
            if (current.0, current.1 - 1) != prev { (current.0, current.1 - 1) }
            else { (current.0, current.1 + 1) }
        },
        Pipe::DownRight => {
            if (current.0 + 1, current.1) != prev { (current.0 + 1, current.1) }
            else { (current.0, current.1 + 1) }
        },
        Pipe::DownLeft => {
            if (current.0 + 1, current.1) != prev { (current.0 + 1, current.1) }
            else { (current.0, current.1 - 1) }
        }

        _ => panic!("Entered illegal pipe!")
    }
}

fn determine_starting_pipe_type(map: &Vec<Vec<Pipe>>, current: (usize, usize)) -> Pipe {
    let left_connected =
        current.1 as i32 - 1 >= 0 &&
            (map[current.0][current.1 - 1] == Pipe::LeftRight ||
                map[current.0][current.1 - 1] == Pipe::UpRight ||
                map[current.0][current.1 - 1] == Pipe::DownRight);

    let up_connected =
        current.0 as i32 - 1 >= 0 &&
            (map[current.0 - 1][current.1] == Pipe::UpDown ||
                map[current.0 - 1][current.1] == Pipe::DownLeft ||
                map[current.0 - 1][current.1] == Pipe::DownRight);

    let right_connected =
        current.1 + 1 < map[0].len() &&
            (map[current.0][current.1 + 1] == Pipe::LeftRight ||
                map[current.0][current.1 + 1] == Pipe::DownLeft ||
                map[current.0][current.1 + 1] == Pipe::UpLeft);

    let down_connected =
        current.0 + 1 < map.len() &&
            (map[current.0 + 1][current.1] == Pipe::UpDown ||
                map[current.0 + 1][current.1] == Pipe::UpLeft ||
                map[current.0 + 1][current.1] == Pipe::UpRight);

    if up_connected && right_connected { Pipe::UpRight }
    else if up_connected && down_connected { Pipe::UpDown }
    else if up_connected && left_connected { Pipe::UpLeft }
    else if left_connected && right_connected { Pipe::LeftRight }
    else if down_connected && right_connected { Pipe::DownRight }
    else if down_connected && left_connected { Pipe::DownLeft }
    else { panic!("Illegal start condition! ")}
}

pub fn solve_a() {
    let map = parse_map(file_reader::read_file_lines("input_data/day10/day10.in"));

    let (y_start, x_start) = find_start_point(&map);

    let mut current = (y_start, x_start);
    let mut prev = (usize::MAX, usize::MAX);
    let mut current_pipe;

    current_pipe = determine_starting_pipe_type(&map, current);

    let mut count = 0;

    while current_pipe != Pipe::StartPipe {
        let current_old = current;

        current = traverse_current(current, prev, current_pipe);
        count += 1;

        current_pipe = map[current.0][current.1];
        prev = current_old;
    }

    println!("{}", f32::ceil((count as f32) / 2.0) as u32);
}

pub fn solve_b() {
    let mut map = parse_map(file_reader::read_file_lines("input_data/day10/day10.in"));

    let (y_start, x_start) = find_start_point(&map);
    let mut visited_coords: HashMap<(usize, usize), Pipe> = HashMap::new();

    let mut current = (y_start, x_start);
    let mut prev = (usize::MAX, usize::MAX);
    let mut current_pipe;

    current_pipe = determine_starting_pipe_type(&map, current);
    visited_coords.insert(current, current_pipe);

    while current_pipe != Pipe::StartPipe {
        let current_old = current;

        current = traverse_current(current, prev, current_pipe);

        current_pipe = map[current.0][current.1];
        visited_coords.insert(current, current_pipe);
        prev = current_old;
    }

    let mut area = 0;

    for y in 0..map.len() {
        let mut should_count = false;
        let mut area_temp = 0;

        for x in 0..map[y].len(){
            if let Some(pipe) = visited_coords.get(&(y, x)){
                match pipe {
                    Pipe::UpDown | Pipe::DownLeft | Pipe::DownRight => {
                        if should_count {
                            area += area_temp;
                            should_count = false;
                            area_temp = 0;
                        } else { should_count = true; }
                    },
                    _ => ()
                }
            }
            else if should_count {
                map[y][x] = Pipe::InsideLoop;
                area_temp += 1
            }
        }
    }

    println!("{}", area);
}