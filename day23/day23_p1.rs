use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use aoc2024::{file, l2d::grid::Grid};

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

fn main() {
    let lines = file::lines_from_file("input.txt");
    // print_gv_file(lines);
    let (node_list, node_map) = create_nodes(lines);
    let mut cycles: HashSet<String> = HashSet::new();
    for node_rc in node_list {
        let node = node_rc.borrow();
        let neighbour_names: Vec<String> =
            node.edges.iter().map(|n| n.borrow().name.clone()).collect();
        for n1_rc in &node.edges {
            let n1 = n1_rc.borrow();
            for n2_rc in &n1.edges {
                let n2 = n2_rc.borrow();
                if neighbour_names.contains(&n2.name) {
                    assert_ne!(node.name, n1.name);
                    assert_ne!(node.name, n2.name);
                    assert_ne!(n1.name, n2.name);
                    let mut cycle_nodes = vec![node.name.clone(), n1.name.clone(), n2.name.clone()];
                    cycle_nodes.sort();
                    cycles.insert(cycle_nodes.join(","));
                };
            }
        }
    }

    let count = cycles
        .iter()
        .filter(|&s| s.chars().nth(0).unwrap() == 't' || s.contains(",t"))
        .count();

    // 2819 too high
    println!("{:?}", count);
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
