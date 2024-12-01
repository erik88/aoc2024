use std::{
    collections::HashMap, fs::File, io::{prelude::*, BufReader}, iter::Map, path::Path
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

// ---

fn main() {
    let lines = lines_from_file("input.txt");
    let length = lines.len();

    let mut left_list: Vec<i32> = vec!(0; length);
    let mut right_list: Vec<i32> = vec!(0; length);

    for line in lines {
        let end_of_first = line.find(' ').unwrap();
        let (left_str, rest_str) = line.split_at(end_of_first);
        let right_str = rest_str.trim();
        let left_number: i32 = left_str.parse().unwrap();
        let right_number: i32 = right_str.parse().unwrap();
        
        left_list.push(left_number);
        right_list.push(right_number);
    }

    left_list.sort();
    right_list.sort();

    {
        let mut left_iter = left_list.clone().into_iter();
        let right_iter = right_list.clone().into_iter();
        let mut sum: i32 = 0;

        for right in right_iter {
            let left = left_iter.next().unwrap();
            let diff = (left - right).abs();
            sum += diff;
        }
        
        println!("Part 1: {}", sum);
    }
    // -------------------------------------------------

    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut current_number = 0;
    let mut current_sum = 0;
    let mut first = true;

    for right in right_list {
        if first {
            current_number = right;
            current_sum = right;
            first = false;
            continue;
        }

        if current_number == right {
            current_sum += right;
        } else {
            map.insert(current_number, current_sum);
            current_number = right;
            current_sum = right;
        }
    }
    map.insert(current_number, current_sum);

    let mut sum: i64 = 0;
    for left in left_list {
        let maybe_match = map.get(&left);
        if let Some(num) = maybe_match {
            sum += i64::from(*num);
        }
    }
    println!("Part 2: {}", &sum);
}
