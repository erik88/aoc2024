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
        let mut prev = -1;
        let mut maybeAsc: Option<bool> = None;
        let mut safe = true;

        for word in split {
            let num: i32 = word.parse().unwrap();
            if prev == -1 {
                prev = num;
                continue;
            }
            if maybeAsc.is_none() {
                if num == prev {
                    println!("Sneaky constant number!");
                    safe = false;
                    break;
                }
                maybeAsc = Some(num > prev);
            }
            let ascend = maybeAsc.unwrap();

            if (ascend && prev > num) || (!ascend && prev < num) {
                safe = false;
                break;
            }

            let diff = (num - prev).abs();
            if diff < 1 || diff > 3 {
                safe = false;
                break;
            }

            prev = num;
        }

        if safe {
            safe_count += 1;
        }
    }

    println!("Part 1: {}", safe_count);
}
