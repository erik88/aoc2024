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
    let nums: Vec<u64> = lines[0].split(' ').into_iter().map(|c| c.parse().unwrap()).collect();

    let n2 = blink_n_times(&nums, 25);

    println!("Part 1: {:?}", n2.len());
}

fn blink_n_times(v: &Vec<u64>, n: u64) -> Vec<u64> {
    if n == 0 {
        return v.clone();
    }
    
    let v2 = blink(v);
    return blink_n_times(&v2, n-1);
}

fn blink(v: &Vec<u64>) -> Vec<u64> {
    let mut v2: Vec<u64> = Vec::new();

    for val in v {
        let val_str = val.to_string();
        if *val == 0 {
            v2.push(1);
        } else if val_str.len() % 2 == 0 {
            let (slice1, slice2) = val_str.split_at(val_str.len()/2);
            v2.push(slice1.parse().unwrap());
            v2.push(slice2.parse().unwrap());
        } else {
            v2.push(val*2024);
        }
    }

    v2
}