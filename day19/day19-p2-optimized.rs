use std::collections::HashMap;

use aoc2024::file;

// ---

struct Patterns {
    vec: Vec<Vec<String>>,
}

impl Patterns {
    #[inline]
    fn get(&self, c: char) -> &[String] {
        self.vec.get(c as usize - 'a' as usize).unwrap()
    }

    fn new(patterns: Vec<String>) -> Patterns {
        let mut vec: Vec<Vec<String>> = vec![Vec::new(); 26];

        for pattern in patterns {
            let index = pattern.chars().nth(0).unwrap() as usize - 'a' as usize;
            vec.get_mut(index).unwrap().push(pattern);
        }

        Patterns {
            vec
        }
    }
}

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

fn check_design(design: &str, patterns: &Patterns, evaluated: &mut HashMap<String, u64>) -> u64 {
    let mut count = 0;

    for pattern in patterns.get(design.chars().nth(0).unwrap()) {
        if design.starts_with(pattern) {
            if pattern.len() == design.len() {
                count += 1;
            } else if let Some(saved_count) = evaluated.get(&design[pattern.len()..]) {
                count += *saved_count;
            } else {
                count += check_design(&design[pattern.len()..], patterns, evaluated);
            }
        }
    }
    evaluated.insert(design.to_string(), count);
    count
}

fn parse_input(lines: Vec<String>) -> (Patterns, Vec<String>) {
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
    (Patterns::new(patterns), designs)
}