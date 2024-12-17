use std::collections::HashMap;

use aoc2024::file;

// ---

#[derive(Debug, Clone)]
struct Registers {
    a: u64,
    b: u64,
    c: u64
}

// 0000000101

/*
 2,4 B <- 3bit(A)
 
 1,3 B <- B bitwise XOR 011
 
 7,5 C <- A / 2^B (A >> B)
 
 0,3 A <- A / 2^3 (A >> 3)
 
 1,4 B <- bitwise XOR 100
 
 4,7 B <- bitwise XOR C
 
 5,5 Output 3bit(B)
 
 3,0 Hoppa till 0, eller bryt om A = 0
*/

/*

Första output:en:

a1, a2, a3
a1, !a2, !a3
!a1 !a2 !a3 

!a1^c1 !a2^c2 !a3^c3

= 0     = 1     = 0

*/

fn main() {
    let lines = file::lines_from_file("input.txt");
    let (_, instructions) = parse_lines(lines);

    // let overmatch_filter = (0b111_100_101_100 & 0b111_111_111) << 3;

    let mut previous_possible = find_four_first(5, 5, 3, 0, &instructions);

    let mut order = instructions.clone();
    order.reverse();
    let mut ptr = 4;

    while ptr < order.len() {
        println!("Finding for {}", order[ptr]);
        print_binary_vec(&previous_possible);
        let possible_solutions = find_single(order[ptr], &instructions);

        let mut new_solutions: Vec<u64> = Vec::new();

        for previous in &previous_possible {
            // How long did it take me to realize that the bit-shift
            // was maximally 7 and not 8?
            // Don't ask.
            let previous_overlay = (previous & 0b1_111_111_111) << 3;
            for possible in &possible_solutions {
                let haystack = possible & 0b1_111_111_111_000;
                if previous_overlay == haystack {
                    new_solutions.push((previous << 3) | (possible & 0b111));
                }
            }
        }
        
        previous_possible = new_solutions.clone();
        ptr += 1;
    }

    let mut solutions = previous_possible.clone();
    solutions.sort();
    println!("Solutions");
    println!("{}", solutions.get(0).unwrap());
    println!("{}", solutions.get(1).unwrap());

    // 2,4,1,3,7,5,0,3,1,4,4,7,5,5,3,0
    // 0,3,5,5,7,4,4,1,3,0,5,7,3,1,4,2
}

fn print_binary_vec(matches: &[u64]) {
    let strs: Vec<String> = matches.iter().map(|v| format!("{:#014b}", v)).collect();
    println!("{:?}",strs);
}

fn find_single(num: u64,instructions: &[u64]) -> Vec<u64> {
    let mut v: Vec<u64> = Vec::new();
    for i in 2_u64.pow(10)..2_u64.pow(13) {
        if test_case(&mut Registers {a: i, b:0, c:0}, &instructions, &vec!(num)) {
            v.push(i);
        }
    }
    v
}

fn find_four_first(num1: u64, num2: u64, num3: u64, num4: u64, instructions: &[u64]) -> Vec<u64> {
    let mut v: Vec<u64> = Vec::new();

    for i in 2_u64.pow(9)..2_u64.pow(12) {
        if test_case(&mut Registers {a: i, b:0, c:0}, &instructions, &vec!(num1, num2, num3, num4)) {
            v.push(i);
        }
    }
    v
}

fn find_five(num1: u64, num2: u64, num3: u64, num4: u64, num5: u64, instructions: &[u64]) {
    for i in 0..100000 {
        if test_case(&mut Registers {a: i, b:0, c:0}, &instructions, &vec!(num1, num2, num3, num4, num5)) {
            println!("Found ({},{},{},{},{})): {:#06b}", num1, num2, num3, num4, num5, i);
        }
    }
}

fn test_number(a_start: u64, expected_output: u8, instructions: &[u64]) -> bool {
    let mut pointer = 0;
    let mut register = Registers { a: a_start, b: 0, c: 0 };

    while pointer < instructions.len() {
        if let Some(out) = execute(&mut pointer, &instructions, &mut register) {
            if out == expected_output {
                return true;
            } else {
                return false;
            }
        }
    }

    return false;
}

fn test_case(register: &mut Registers, instructions: &[u64], expected: &[u64]) -> bool {
    let mut pointer = 0;
    let mut ptr = 0;

    while pointer < instructions.len() {
        if let Some(out) = execute(&mut pointer, &instructions, register) {
            if expected[ptr] == out.into() {
                ptr += 1;

                if expected.len() == ptr {
                    return true;
                }
            } else {
                return false;
            }
        }
    }

    return false;
}

fn execute(pointer: &mut usize, instructions: &[u64], register: &mut Registers) -> Option<u8> {
    let opcode = instructions[*pointer];
    let operand = instructions[*pointer+1];

    match opcode {
        0 => {
            register.a = register.a / 2_u64.pow(combo(operand, &register).try_into().unwrap());
            m(pointer);
        },
        1 => {
            register.b = register.b ^ operand;
            m(pointer);
        },
        2 => {
            register.b = combo(operand, &register) % 8;
            m(pointer);
        },
        3 => {
            if register.a != 0 {
                *pointer = operand.try_into().unwrap();
            } else {
                m(pointer);
            }
        },
        4 => {
            register.b = register.b ^ register.c;
            m(pointer);
        },
        5 => {
            m(pointer);
            return Some((combo(operand, &register) % 8).try_into().unwrap());
        },
        6 => {
            register.b = register.a / 2_u64.pow(combo(operand, &register).try_into().unwrap());
            m(pointer);
        },
        7 => {
            register.c = register.a / 2_u64.pow(combo(operand, &register).try_into().unwrap());
            m(pointer);
        },
        _ => {
            panic!("Unknown opcode {}", opcode);
        }
    }

    return None;
}

fn m(pointer: &mut usize) {
    *pointer += 2;
}

fn combo(operand: u64, register: &Registers) -> u64 {
    match operand {
        0..=3 => {
            operand
        },
        4 => register.a,
        5 => register.b,
        6 => register.c,
        _ => panic!("Invalid combo operator {}", operand)
    }
}

fn parse_lines(lines: Vec<String>) -> (Registers, Vec<u64>) {
    let reg = Registers {
        a: lines[0].split_once(':').unwrap().1.trim().parse().unwrap(),
        b: lines[1].split_once(':').unwrap().1.trim().parse().unwrap(),
        c: lines[2].split_once(':').unwrap().1.trim().parse().unwrap(),
    };

    let comma_list = lines[4].split_once(':').unwrap().1.trim();
    let v: Vec<u64> = comma_list.split(',').into_iter().map(|s| s.parse().unwrap()).collect();

    (reg, v)
}
