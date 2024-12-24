use std::{cell::RefCell, fs, rc::Rc};

use aoc2024::file;

// ---

struct Wire {
    name: String,
    value: Option<bool>,
    from: Option<Rc<RefCell<Gate>>>
}

struct Gate {
    in1: Rc<RefCell<Wire>>,
    in2: Rc<RefCell<Wire>>,
    out: Rc<RefCell<Wire>>,
    op: Operation
}

#[derive(PartialEq)]
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

    fn name(&self) -> &str {
        match self {
            Operation::And => "AND",
            Operation::Or => "OR",
            Operation::Xor => "XOR"
        }
    }
}

fn main() {
    let lines = file::lines_from_file("input.txt");

    let (mut wires, mut gates) = parse_lines(lines);
    
    // Do 1111111111111111111111 + 000000000000000000000
    wires.iter()
        .filter(|w_rc| w_rc.borrow().name.chars().nth(0).unwrap() == 'x')
        .for_each(|w_rc| w_rc.borrow_mut().value = Some(true));

    wires.iter()
        .filter(|w_rc| w_rc.borrow().name.chars().nth(0).unwrap() == 'y')
        .for_each(|w_rc| w_rc.borrow_mut().value = Some(false));

    // Graph is messed up by swap.
    generate_dot_file(&gates);

    swap("z10","gpr", &mut wires, &mut gates);
    swap("z21","nks", &mut wires, &mut gates);
    swap("cpm","krs", &mut wires, &mut gates);
    swap("z33","ghp", &mut wires, &mut gates);

    let mut answer = vec!("z10","gpr","z21","nks","cpm","krs","z33","ghp");
    answer.sort();

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
    let mut bin_str = String::new();
    wires.into_iter()
        .filter(|w| w.borrow().name.chars().nth(0).unwrap() == 'z')
        .map(|w| w.borrow().value.unwrap())
        .for_each(|b| bin_str += if b { "1" } else { "0" });

    // for i in 0..=44 {
    //     let is = if i < 10 { format!("0{}",i) } else { i.to_string() };
    //     let from_name = format!("x{}", is);
    //     let gates = get_gates_coming_from(from_name.clone(), &gates);
    //     let g1: Vec<&Rc<RefCell<Gate>>> = gates.iter().filter(|g| g.borrow().op == Operation::Xor).collect();
    //     let g2: Vec<&Rc<RefCell<Gate>>> = gates.iter().filter(|g| g.borrow().op == Operation::And).collect();
    //     if g1.len() != 1 {
    //         println!("Missing XOR: {}", &from_name);
    //     }
    //     if g2.len() != 1 {
    //         println!("Missing AND: {}", from_name);
    //     }
    // }

    //let result = usize::from_str_radix(&bin_str, 2).unwrap();

    //  111111111111111111111111111111111111111111111
    // +000000000000000000000000000000000000000000000
    // 1000001000000111111111111000000000001111111111

    // After first swap
    // 1000000111111000000000000000000000001111111111

    // x/y 10
    // x/y 33
    // x/y 40
    // x/y 44

    // By graph inspection
    // z21/nks
    println!("Addition: {}", bin_str);
    println!("Answer: {}", answer.join(","));
}

fn generate_dot_file(gates: &Vec<Rc<RefCell<Gate>>>) {
    let mut s = String::new();
    s += "digraph mygraph {\n";
    for gate_rc in gates {
        let gate = gate_rc.borrow();
        let fromop1 = if let Some(fr) = &gate.in1.borrow().from {
            fr.borrow().op.name().to_string()
        } else {
            "".to_string()
        };
        let fromop2 = if let Some(fr) = &gate.in2.borrow().from {
            fr.borrow().op.name().to_string()
        } else {
            "".to_string()
        };
        s += &format!("{}{} -> {}{}\n", gate.in1.borrow().name, fromop1, gate.out.borrow().name, gate.op.name());
        s += &format!("{}{} -> {}{}\n", gate.in2.borrow().name, fromop2, gate.out.borrow().name, gate.op.name());
    }
    s += "}";
    fs::write("graph.gv", s).unwrap();
}

fn swap(w1_name: &str, w2_name: &str, wires: &Vec<Rc<RefCell<Wire>>>, gates: &[Rc<RefCell<Gate>>]) {
    let gate1 = gates.iter().find(|g| g.borrow().out.borrow().name == w1_name).unwrap();
    let gate2 = gates.iter().find(|g| g.borrow().out.borrow().name == w2_name).unwrap();
    {
        let gate1_out = Rc::clone(&gate1.borrow().out);
        gate1.borrow_mut().out = Rc::clone(&gate2.borrow().out);
        gate2.borrow_mut().out = gate1_out;
    }
    let w1 = get_wire_by_name(w1_name, wires);
    let w2 = get_wire_by_name(w1_name, wires);
    let w1_from = Rc::clone(&gate1);
    w1.borrow_mut().from = Some(Rc::clone(&gate2));
    w2.borrow_mut().from = Some(w1_from);
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

            w3.borrow_mut().from = Some(Rc::clone(&gate));
        }
    }

    (wires, gates)
}

fn get_gates_coming_from(name: String, gates: &Vec<Rc<RefCell<Gate>>>) -> Vec<Rc<RefCell<Gate>>> {
    let mut v = Vec::new();
    for gate in gates {
        if gate.borrow().in1.borrow().name == name || gate.borrow().in2.borrow().name == name {
            v.push(Rc::clone(gate));
        }
    }
    v
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
        })));
    }
}