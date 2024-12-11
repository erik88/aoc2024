use std::{
    collections::HashMap, fs::File, io::{prelude::*, BufReader}, path::Path
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

// ---

#[derive(Hash, Eq, PartialEq)]
struct HashKey {
    num: u64,
    steps: u8,
}

fn main() {
    let lines = lines_from_file("input.txt");
    let nums: Vec<u64> = lines[0].split(' ').into_iter().map(|c| c.parse().unwrap()).collect();
    let mut hmap: HashMap<HashKey, u64> = HashMap::new();
    let mut sum = 0;

    for num in nums {
        sum += blink_n_times(num, 75, &mut hmap);
    }

    println!("Part 2: {:?}", sum);
}

fn blink_n_times(num: u64, steps: u8, hmap: &mut HashMap<HashKey, u64>) -> u64 {
    if steps == 0 {
        return 1;
    }
    if num < 20000 {
        if let Some(count) = hmap.get(&HashKey { num, steps }) {
            return *count;
        }
    } 

    let mut sum = 0;
    if num == 0 {
        sum += blink_n_times(1, steps - 1, hmap);
    } else {
        let num_str = num.to_string();
        if num_str.len() % 2 == 0 {
            let (slice1, slice2) = num_str.split_at(num_str.len()/2);
            sum += blink_n_times(slice1.parse().unwrap(), steps - 1, hmap);
            sum += blink_n_times(slice2.parse().unwrap(), steps - 1, hmap);
         } else {
            sum += blink_n_times(num*2024, steps - 1, hmap);
         }
    }

    hmap.insert(HashKey { num, steps }, sum);

    return sum;
}