use std::{
    fs::File, io::{prelude::*, BufReader}, path::Path
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
    let mut sum : i64 = 0;

    for line in lines {
        sum += get_number_for_line(&line);
    }
    println!("Part 1: {}", sum);
}

fn get_number_for_line(s: &str) -> i64 {
    let mut state = 0;
    let mut num_buf = String::new();
    let mut num1: i64 = 0;
    let mut num2: i64;
    let mut sum: i64 = 0;

    for c in s.chars() {
        if state == 0 && c == 'm' {
            state = 1;
        } else if state == 1 && c == 'u' {
            state = 2;
        } else if state == 2 && c == 'l' {
            state = 3;
        } else if state == 3 && c == '(' {
            state = 4;
        } else if state == 4 {
            if c.is_numeric() {
                num_buf.push(c);
            } else if c == ',' && num_buf.len() > 0 {
                num1 = num_buf.parse().unwrap();
                num_buf.clear();
                state = 5;
            } else {
                num1 = 0;
                state = 0;
                num_buf.clear();
            }
        } else if state == 5 {
            if c.is_numeric() {
                num_buf.push(c);
            } else if c == ')' && num_buf.len() > 0 {
                num2 = num_buf.parse().unwrap();
                num_buf.clear();

                sum += num1*num2;

                num1 = 0;
                state = 0;
            } else {
                num1 = 0;
                state = 0;
                num_buf.clear();
            }
        } else {
            state = 0;
            num1 = 0;
        }
    }

    return sum;
}