use std::{
    cmp::{self, Ordering}, collections::HashMap, fs::File, io::{prelude::*, BufReader}, iter, path::Path
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

    let (map, rows) = parse_input(lines); 

    let mut sum_p1: i32 = 0;
    let mut sum_p2: i32 = 0;

    for row in rows {
        if check_row(&row, &map) {
            sum_p1 += row.get((row.len()-1)/2).unwrap();
        } else {
            let fixed_row = fix_row(&row, &map);
            sum_p2 += fixed_row.get((fixed_row.len()-1)/2).unwrap();
        }
    }

    println!("Part 1: {}", sum_p1);
    println!("Part 2: {}", sum_p2);
}

fn check_row(row: &Vec<i32>, map: &HashMap<i32, Vec<i32>>) -> bool {
    for i in (0..row.len()).rev() {
        let item1 = row.get(i).unwrap();

        if let Some(rules1) = map.get(item1) {
            for j in 0..i {
                let item2 = row.get(j).unwrap();
                if rules1.contains(item2) {
                    return false
                }
            }       
        }
    }
    true
}

fn fix_row(row_in: &Vec<i32>, map: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut row = row_in.clone();
    row.sort_by(|a, b| {
        if let Some(rules) = map.get(a) {
            if rules.contains(b) {
                return Ordering::Less;
            }
        } else if let Some(rules) = map.get(b) {
            if rules.contains(a) {
                return Ordering::Greater;
            }
        }
        return Ordering::Equal;
    });
    row
}

fn parse_input(lines: Vec<String>) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut stage = 1;
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut rows: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        if stage == 1 {
            if line.is_empty() {
                stage = 2;
                continue;
            }
            let mut split = line.splitn(2, '|');
            let first: i32 = split.next().unwrap().parse().unwrap();
            let second: i32 = split.next().unwrap().parse().unwrap();
            if !map.contains_key(&first) {
                map.insert(first, vec!(second));
            } else {
                let vec: &mut Vec<i32> = map.get_mut(&first).unwrap();
                vec.push(second);
            }
        } else {
            rows.push(line.split(',').into_iter().map(|s| {
                if let Some(num) = s.parse::<i32>().ok() {
                    return num
                } else {
                    println!("Invalid integer: {}", s);
                    panic!("Invalid line\n{}", line);
                }
            }
            ).collect())
        }
    }

    (map, rows)
}