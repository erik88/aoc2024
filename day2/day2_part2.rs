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
    let mut safe_count = 0;

    for line in lines {
        let split = line.split(' ');
        let nums: Vec<i32> = split.map(|w| w.parse().unwrap()).collect();
        let mut safe = false;

        for index in 0..nums.len() {
            let mut nums_x = nums.clone();
            nums_x.remove(index);
            if check_levels(nums_x) {
                safe = true;
                break;
            }
        }

        if safe {
            safe_count += 1;
        }
    }

    println!("Part 2: {}", safe_count);
}

fn check_levels(nums: Vec<i32>) -> bool {
    let mut prev = -1;
    let mut maybe_asc: Option<bool> = None;

    for num in nums {
        if prev == -1 {
            prev = num;
            continue;
        }
        if maybe_asc.is_none() {
            if num == prev {
                return false;
            }
            maybe_asc = Some(num > prev);
        }
        let ascend = maybe_asc.unwrap();

        if (ascend && prev > num) || (!ascend && prev < num) {
            return false;
        }

        let diff = (num - prev).abs();
        if diff < 1 || diff > 3 {
            return false;
        }

        prev = num;
    }

    return true;
}
