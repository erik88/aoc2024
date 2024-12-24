use std::{cell::RefCell, rc::Rc};

use aoc2024::file;

// ---

struct Wire {
    name: String,
    value: Option<bool>,
    from: Option<Rc<RefCell<Gate>>>,
    to: Option<Rc<RefCell<Gate>>>,
}

struct Gate {
    in1: Rc<RefCell<Wire>>,
    in2: Rc<RefCell<Wire>>,
    out: Rc<RefCell<Wire>>,
    op: Operation
}

enum Operation {
    And, Or, Xor
}
impl Operation {
    fn from_str(s: &str) -> Operation {
        match s {
            "AND" => Operation::And,
            "OR" => Operation::Or,
            "XOR" => Operation::Xor,
            _ => panic!("Could not parse operation '{}'", s),
        }
    }

    fn execute(&self, v1: bool, v2: bool) -> bool {
        match self {
            Operation::And => v1 && v2,
            Operation::Or => v1 || v2,
            Operation::Xor => v1 ^ v2,
        }
    }
}

fn main() {
    let lines = file::lines_from_file("input.txt");

    let (mut wires, gates) = parse_lines(lines);

    loop {
        let mut resolve_count = 0;
        for gate_rc in &gates {
            let gate = gate_rc.borrow();
            let in1 = gate.in1.borrow().value;
            let in2 = gate.in2.borrow().value;
            let out_val = &mut gate.out.borrow_mut().value;

            if out_val.is_none() {
                if in1.is_some() && in2.is_some() {
                    *out_val = Some(gate.op.execute(in1.unwrap(), in2.unwrap()));
                    resolve_count += 1;
                }
            }
        }
        if resolve_count == 0 {
            break;
        }
    }

    wires.sort_by(|w1, w2| w2.borrow().name.cmp(&w1.borrow().name));
    let mut binStr = String::new();
    wires.into_iter()
        .filter(|w| w.borrow().name.chars().nth(0).unwrap() == 'z')
        .map(|w| w.borrow().value.unwrap())
        .for_each(|b| binStr += if b { "1" } else { "0" });

    let result = usize::from_str_radix(&binStr, 2).unwrap();

    println!("Answer: {}", result);
}

fn parse_lines(lines: Vec<String>) -> (Vec<Rc<RefCell<Wire>>>, Vec<Rc<RefCell<Gate>>>) {
    let mut state = 0;
    let mut wires: Vec<Rc<RefCell<Wire>>> = Vec::new();
    let mut gates: Vec<Rc<RefCell<Gate>>> = Vec::new();

    for line in lines {
        if state == 0 {
            if line.is_empty() {
                state = 1;
                continue;
            }
            let (name, val) = line.split_once(": ").unwrap();
            wires.push(Rc::new(RefCell::new(Wire {
                name: name.to_string(),
                value: Some(val == "1"), 
                to: None,
                from: None,
            })));
        } else {
            let mut split = line.split(" ");
            let w1n = split.next().unwrap();
            let op = split.next().unwrap();
            let w2n = split.next().unwrap();
            split.next(); // "->"
            let w3n = split.next().unwrap();

            ensure_wire_exist(w1n, &mut wires);
            ensure_wire_exist(w2n, &mut wires);
            ensure_wire_exist(w3n, &mut wires);

            let w1 = get_wire_by_name(w1n, &wires);
            let w2 = get_wire_by_name(w2n, &wires);
            let w3 = get_wire_by_name(w3n, &wires);

            let gate = Rc::new(RefCell::new(
                Gate {
                    in1: Rc::clone(&w1),
                    in2: Rc::clone(&w2),
                    out: Rc::clone(&w3),
                    op: Operation::from_str(op),
                }
            ));
            gates.push(Rc::clone(&gate));

            w1.borrow_mut().to = Some(Rc::clone(&gate));
            w2.borrow_mut().to = Some(Rc::clone(&gate));
            w3.borrow_mut().from = Some(Rc::clone(&gate));
        }
    }

    (wires, gates)
}

fn get_wire_by_name(name: &str, wires: &Vec<Rc<RefCell<Wire>>>) -> Rc<RefCell<Wire>> {
    Rc::clone(wires.iter().find(|w| w.borrow().name == name).unwrap())
}

fn ensure_wire_exist(name: &str, wires: &mut Vec<Rc<RefCell<Wire>>>) {
    if !wires.iter().any(|w| w.borrow().name == name) {
        wires.push(Rc::new(RefCell::new(Wire {
            name: name.to_string(),
            value: None,
            from: None,
            to: None
        })));
    }
}