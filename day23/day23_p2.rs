use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use aoc2024::file;

// ---

struct Node {
    name: String,
    edges: Vec<Rc<RefCell<Node>>>,
}
impl Node {
    fn new(name: &str) -> Node {
        Node {
            name: name.to_owned(),
            edges: Vec::new(),
        }
    }
}

#[derive(Clone)]
struct Cycle {
    nodes: Vec<Rc<RefCell<Node>>>,
}
impl Cycle {
    fn new_three(n1: &Rc<RefCell<Node>>, n2: &Rc<RefCell<Node>>, n3: &Rc<RefCell<Node>>) -> Cycle {
        Cycle {
            nodes: vec![Rc::clone(n1), Rc::clone(n2), Rc::clone(n3)],
        }
    }
    fn hash(&self) -> String {
        let mut v: Vec<String> = self
            .nodes
            .iter()
            .map(|n| n.borrow().name.to_string())
            .collect();
        v.sort();
        v.join(",")
    }
}

fn main() {
    let lines = file::lines_from_file("input.txt");
    // print_gv_file(lines);
    let (node_list, _) = create_nodes(lines);
    let mut cycle_hashes: HashSet<String> = HashSet::new();
    let mut cycles: Vec<Cycle> = Vec::new();
    for node_rc in node_list {
        let node = node_rc.borrow();
        let neighbour_names: Vec<String> =
            node.edges.iter().map(|n| n.borrow().name.clone()).collect();
        for n1_rc in &node.edges {
            let n1 = n1_rc.borrow();
            for n2_rc in &n1.edges {
                let n2 = n2_rc.borrow();
                if neighbour_names.contains(&n2.name) {
                    let cycle = Cycle::new_three(&node_rc, n1_rc, n2_rc);
                    cycle_hashes.insert(cycle.hash());
                    cycles.push(cycle);
                };
            }
        }
    }

    let mut biggest_cycle_hash = String::new();
    while let Some(c) = cycles.get(0) {
        biggest_cycle_hash = c.hash();
        println!("{}", c.nodes.len());
        cycles = extend_all(cycles);
    }

    // 2819 too high
    println!("{}", biggest_cycle_hash);
}

fn extend_all(cycles: Vec<Cycle>) -> Vec<Cycle> {
    let mut bigger_cycles: Vec<Cycle> = Vec::new();
    let mut bigger_cycles_visited: HashSet<String> = HashSet::new();
    for cycle in cycles {
        let new_cycles = extend(&cycle);
        for nc in new_cycles {
            if bigger_cycles_visited.insert(nc.hash()) {
                bigger_cycles.push(nc);
            }
        }
    }
    bigger_cycles
}

fn extend(cycle: &Cycle) -> Vec<Cycle> {
    let neighbours = get_neighbours(&cycle);
    let mut res: Vec<Cycle> = Vec::new();

    for n in neighbours {
        if is_connected_to_all(&n, cycle) {
            let mut bigger_cycle = cycle.clone();
            bigger_cycle.nodes.push(Rc::clone(&n));
            res.push(bigger_cycle);
        }
    }
    res
}

fn is_connected_to_all(n: &Rc<RefCell<Node>>, cycle: &Cycle) -> bool {
    let n_name = n.borrow().name.clone();
    for c in &cycle.nodes {
        if !c.borrow().edges.iter().any(|e| e.borrow().name == n_name) {
            return false;
        }
    }
    return true;
}

fn get_neighbours(cycle: &Cycle) -> Vec<Rc<RefCell<Node>>> {
    let mut visited: HashSet<String> = HashSet::new();
    for n in &cycle.nodes {
        visited.insert(n.borrow().name.clone());
    }
    let mut neighbours: Vec<Rc<RefCell<Node>>> = Vec::new();
    for n in &cycle.nodes {
        for n2 in &n.borrow().edges {
            if visited.insert(n2.borrow().name.clone()) {
                neighbours.push(Rc::clone(n2));
            }
        }
    }
    neighbours
}

fn create_nodes(
    lines: Vec<String>,
) -> (Vec<Rc<RefCell<Node>>>, HashMap<String, Rc<RefCell<Node>>>) {
    let mut node_map: HashMap<String, Rc<RefCell<Node>>> = HashMap::new();

    for line in lines {
        let (left, right) = line.split_once("-").unwrap();
        let n1 = get_or_create_node(left, &mut node_map);
        let n2 = get_or_create_node(right, &mut node_map);

        n1.borrow_mut().edges.push(Rc::clone(&n2));
        n2.borrow_mut().edges.push(n1);
    }

    let v = node_map
        .iter()
        .map(|(_, n)| Rc::clone(n))
        .collect::<Vec<Rc<RefCell<Node>>>>();

    (v, node_map)
}

fn get_or_create_node(
    name: &str,
    nodes: &mut HashMap<String, Rc<RefCell<Node>>>,
) -> Rc<RefCell<Node>> {
    if let Some(node_ref_cell) = nodes.get(name) {
        Rc::clone(node_ref_cell)
    } else {
        let node_rc = Rc::new(RefCell::new(Node::new(name)));
        nodes.insert(name.to_string(), Rc::clone(&node_rc));
        node_rc
    }
}

// fn print_gv_file(lines: Vec<String>) {
//     let mut gv = String::new();
//     gv += &"strict graph {\n";
//     for line in lines {
//         let (left, right) = line.split_once("-").unwrap();
//         gv = gv + &format!("{} -- {}\n", left, right);
//     }
//     gv += &"}";

//     std::fs::write("graph.gv", gv).unwrap();
// }
