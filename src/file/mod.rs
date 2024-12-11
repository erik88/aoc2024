use std::{
    fs::File, io::{prelude::*, BufReader}, path::Path
};

pub fn string_from_file(filename: impl AsRef<Path>) -> String {
    std::fs::read_to_string(filename).expect("no such file")
}

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn chars_from_file(filename: impl AsRef<Path>) -> Vec<Vec<char>> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|l| l.chars().collect())
        .collect()
}

pub fn digits_from_file(filename: impl AsRef<Path>) -> Vec<Vec<u32>> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|l| l.chars().into_iter().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}