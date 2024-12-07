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

struct Equation {
    result: i64,
    parts: Vec<i64>,
}

fn main() {
    let lines = lines_from_file("input.txt");

    let equations: Vec<Equation> = lines.into_iter().map(|l| parse_equation(&l)).collect();
    let mut result = 0;
    for eq in equations {
        if can_be_solved(&eq) {
            result += eq.result;
        }
    }

    println!("Part 2: {}", result);
}

fn parse_equation(line: &str) -> Equation {
    let split_pos = line.find(':').unwrap();
    Equation {
        result: line[0..split_pos].parse().unwrap(),
        parts: line[split_pos+1..].split_whitespace().map(|word| word.parse().unwrap()).collect(),
    }
}

#[derive(Clone, Debug)]
enum Operator {
    Add,
    Multiply,
    Concat
}

fn can_be_solved(eq: &Equation) -> bool {
    let spaces = eq.parts.len() - 1;

    let ops_collection: Vec<Vec<Operator>> = get_all_combinations(spaces.try_into().unwrap());


    for ops in ops_collection {
        let mut iter = eq.parts.iter();
        let mut res = *iter.next().unwrap();
        for op in ops {
            let part = *iter.next().unwrap();
            res = match op {
                Operator::Add => res + part,
                Operator::Multiply => res * part,
                Operator::Concat => (res.to_string() + &part.to_string()).parse::<i64>().unwrap(),
            };

            if res > eq.result {
                break
            }
        }

        if res == eq.result {
            return true
        }
    }

    false
}

fn get_all_combinations(spaces: i32) -> Vec<Vec<Operator>> {
    if spaces == 1 {
        return vec!(vec!(Operator::Add), vec!(Operator::Multiply), vec!(Operator::Concat));
    }
    let combos = get_all_combinations(spaces - 1);
    let mut l1: Vec<Vec<Operator>> = combos.clone().into_iter().map(|mut v| { v.push(Operator::Add); return v}).collect();
    let mut l2: Vec<Vec<Operator>> = combos.clone().into_iter().map(|mut v| { v.push(Operator::Multiply); return v}).collect();
    let mut l3: Vec<Vec<Operator>> = combos.clone().into_iter().map(|mut v| { v.push(Operator::Concat); return v}).collect();
    l1.append(&mut l2);
    l1.append(&mut l3);
    return l1;
}