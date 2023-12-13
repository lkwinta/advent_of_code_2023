use crate::file_reader;
#[allow(dead_code)]
fn check_counts(record: &str, counts: &Vec<i32>) -> bool {
    let mut str_iterator = 0;
    let mut broken_count;

    for count in counts {
        broken_count = 0;

        while str_iterator < record.len() && record.chars().nth(str_iterator).unwrap() == '.' {
            str_iterator += 1;
        }

        while str_iterator < record.len() && record.chars().nth(str_iterator).unwrap() == '#' {
            str_iterator += 1;
            broken_count += 1;
        }

        if broken_count != *count {
            return false;
        }
    }

    while str_iterator < record.len() {
        if record.chars().nth(str_iterator).unwrap() == '#' {
            return  false;
        }
        str_iterator += 1;
    }

    return true;
}
#[allow(dead_code)]
fn count_possibilities(record: &str, counts: &Vec<i32>, index: usize) -> i32{
    let mut str_iterator = index;
    while str_iterator < record.len() && record.chars().nth(str_iterator).unwrap() != '?' {
        str_iterator += 1;
    }

    return if str_iterator == record.len() {
        check_counts(record, counts) as i32
    } else {
        let mut record_string = String::from(record);
        record_string.replace_range(str_iterator..str_iterator + 1, ".");

        let c1 = count_possibilities(record_string.as_str(), counts, str_iterator + 1);

        record_string.replace_range(str_iterator..str_iterator + 1, "#");
        let c2 = count_possibilities(record_string.as_str(), counts, str_iterator + 1);

        c1 + c2
    }
}
#[allow(dead_code)]
pub fn solve_a() {
    let lines = file_reader::read_file_lines("input_data/day12/day12.in");
    let mut sum = 0;

    for line in lines {
        let mut line_split = line.split(' ');
        let record_data = line_split.nth(0).unwrap();
        let proper_data = line_split
            .nth(0).unwrap()
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect::<Vec<i32>>();

        sum += count_possibilities(record_data, &proper_data, 0);
    }

    println!("{}", sum);
}
#[allow(dead_code)]
pub fn solve_b() {
    let lines = file_reader::read_file_lines("input_data/day12/day12_test.in");
    let mut sum = 0;

    for line in lines {
        let mut line_split = line.split(' ');
        let record_data = line_split.nth(0).unwrap();
        let proper_data = line_split
            .nth(0).unwrap()
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect::<Vec<i32>>();

        let mut new_record_data = String::from("");
        let mut new_proper_data : Vec<i32> = Vec::new();

        for _ in 0..5 {
            new_record_data.push_str(record_data);
            new_proper_data.extend(proper_data.iter());
        }

        println!("{} -> {:?}", new_record_data, new_proper_data);
        sum += count_possibilities(new_record_data.as_str(), &new_proper_data, 0);
    }

    println!("{}", sum);
}