use std::collections::{HashMap, HashSet, VecDeque};

use aoc2024::file;

// ---

fn main() {
    let lines = file::lines_from_file("input.txt");
    // println!("{:#032b}", 16777216);
    // *64 = << 6 (64 = 2^6));
    // /32 = >> 5 (32 = 2^5)
    // *2048 = << 11 (2048 = 2^11)
    // Prune: keep the last 24 bits
    // This might be a dead-end...

    let init_nums: Vec<usize> = /*vec![123];*/ lines
                                           .into_iter()
                                           .filter(|l| !l.is_empty())
                                           .map(|l| l.parse::<usize>().unwrap())
                                           .collect();

    let mut total: HashMap<String, usize> = HashMap::new();

    for num in init_nums {
        add_to_total(num, &mut total);
    }

    let mut max = 0;
    let mut max_key = String::new();
    for (key, val) in total {
        if val > max {
            max = val;
            max_key = key.clone();
        }
    }

    println!("Most bananas: {}, from {}", max, max_key);
}

fn add_to_total(first_secret: usize, total: &mut HashMap<String, usize>) {
    let mut curr = first_secret;
    let mut latest_diffs = VecDeque::new();
    let mut visited: HashSet<String> = HashSet::new();
    for _ in 0..2000 {
        // Algorithm
        let s1 = ((curr * 64) ^ curr) % 16777216;
        let s2 = ((s1 / 32) ^ s1) % 16777216;
        let s3 = ((s2 * 2048) ^ s2) % 16777216;

        let ones = s3 % 10;
        let diff: i64 = ones as i64 - curr as i64 % 10;

        latest_diffs.push_back(diff);
        if latest_diffs.len() == 4 {
            let hash = latest_diffs
                .iter()
                .map(|&i| i.to_string())
                .collect::<Vec<String>>()
                .join(",");
            if visited.insert(hash.clone()) {
                if let Some(entry) = total.get_mut(&hash) {
                    *entry += ones;
                } else {
                    total.insert(hash, ones);
                }
            }
            latest_diffs.pop_front();
        }
        curr = s3;
    }
}
