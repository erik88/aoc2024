use aoc2024::file;

// ---

#[derive(Debug, Clone)]
struct Registers {
    a: i64,
    b: i64,
    c: i64
}

fn main() {
    run_test_cases();
    let lines = file::lines_from_file("input.txt");
    let (mut register, instructions) = parse_lines(lines);
    let out = run_case(&mut register, &instructions);

    println!("{:?}", register);

    // Not right 0,6,0,1,7,2,3,7,5
    println!("Answer: {}", out);
}

fn assert_case(case_name: &str, register: &mut Registers, instructions: &[i64], ass_register: Registers, ass_out: Option<String>) {
    let out = run_case(register, instructions);
    if ass_register.a > -1 {
        assert!(ass_register.a == register.a, "{}", case_name);
    }
    if ass_register.b > -1 {
        assert!(ass_register.b == register.b, "{}: Expected b to be {}, but was {}", case_name, ass_register.b, register.b);
    }
    if ass_register.c > -1 {
        assert!(ass_register.c == register.c, "{}", case_name);
    }
    if let Some(ass_out_val) = ass_out {
        assert!(out.eq(&ass_out_val), "{}", case_name);
    }
}

fn run_case(register: &mut Registers, instructions: &[i64]) -> String {
    let mut pointer = 0;
    let mut out = String::new();

    while pointer < instructions.len() {
        execute(&mut pointer, &instructions, register, &mut out);
    }

    out
}

fn execute(pointer: &mut usize, instructions: &[i64], register: &mut Registers, output: &mut String) {
    let opcode = instructions[*pointer];
    let operand = instructions[*pointer+1];

    match opcode {
        0 => {
            register.a = register.a / 2_i64.pow(combo(operand, &register).try_into().unwrap());
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
            let next = (combo(operand, &register) % 8).to_string();
            if output.len() == 0 {
                *output = next;
            } else {
                *output = format!("{},{}", output, next);
            }
            m(pointer);
        },
        6 => {
            register.b = register.a / 2_i64.pow(combo(operand, &register).try_into().unwrap());
            m(pointer);
        },
        7 => {
            register.c = register.a / 2_i64.pow(combo(operand, &register).try_into().unwrap());
            m(pointer);
        },
        _ => {
            panic!("Unknown opcode {}", opcode);
        }
    }
}

fn m(pointer: &mut usize) {
    *pointer += 2;
}

fn combo(operand: i64, register: &Registers) -> i64 {
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

fn parse_lines(lines: Vec<String>) -> (Registers, Vec<i64>) {
    let reg = Registers {
        a: lines[0].split_once(':').unwrap().1.trim().parse().unwrap(),
        b: lines[1].split_once(':').unwrap().1.trim().parse().unwrap(),
        c: lines[2].split_once(':').unwrap().1.trim().parse().unwrap(),
    };

    let comma_list = lines[4].split_once(':').unwrap().1.trim();
    let v: Vec<i64> = comma_list.split(',').into_iter().map(|s| s.parse().unwrap()).collect();

    (reg, v)
}

// ------------------------------------------------------

fn run_test_cases() {
    assert_case("Case 1", &mut Registers {a:0, b:0, c:9}, &vec!(2,6),
        Registers { a: -1, b: 1, c: -1}, None
    );
    assert_case("Case 2", &mut Registers {a:10, b:0, c:0}, &vec!(5,0,5,1,5,4),
        Registers { a: -1, b: -1, c: -1}, Some("0,1,2".to_string())
    );
    assert_case("Case 3", &mut Registers {a:2024, b:0, c:0}, &vec!(0,1,5,4,3,0),
        Registers { a: 0, b: -1, c: -1}, Some("4,2,5,6,7,7,7,7,3,1,0".to_string())
    );
    assert_case("Case 4", &mut Registers {a:0, b:29, c:0}, &vec!(1,7),
        Registers { a: -1, b: 26, c: -1}, None
    );
    assert_case("Case 5", &mut Registers {a:0, b:2024, c:43690}, &vec!(4,0),
        Registers { a: -1, b: 44354, c: -1}, None
    );

    // 6 och 7 testas aldrig av något av test-casen...

    assert_case("Test 6", &mut Registers {a:12, b:18, c:0}, &vec!(6,2),
    // a / 2^combo
    // INTE "^"
    // FML
    // .pow
        Registers { a: -1, b: 3, c: -1}, None
    );
}