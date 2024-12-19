use std::collections::HashMap;

use aoc2024::file;

// ---

fn main() {
    let lines = file::lines_from_file("input.txt");
    let (patterns, designs) = parse_input(lines);
    let mut evaluated: HashMap<String, u64> = HashMap::new();

    let mut working_designs = 0;
    for design in designs {
        working_designs += check_design(&design, &patterns, &mut evaluated);
    }

    println!("Answer: {}", working_designs);
}

fn check_design(design: &str, patterns: &[String], evaluated: &mut HashMap<String, u64>) -> u64 {
    if design.len() == 0 {
        return 1;
    }

    if let Some(count) = evaluated.get(design) {
        return *count;
    }

    let mut count = 0;
    for pattern in patterns {
        if design.starts_with(pattern) {
            if &design[pattern.len()..].len() == 1 {
                count += 1;
            } else {
                count += check_design(&design[pattern.len()..], patterns, evaluated);
            }
        }
    }
    evaluated.insert(design.to_string(), count);
    count
}

fn parse_input(lines: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut state = 0;
    let mut first_rows = String::new();
    let mut designs: Vec<String> = Vec::new();
    for line in lines {
        if state == 0 {
            if line.is_empty() {
                state = 1;
            } else {
                first_rows += &line;
            }
        } else {
            designs.push(line);
        }
    }

    let patterns: Vec<String> = first_rows.split(", ").map(|s| s.to_string()).collect();
    (patterns, designs)
}