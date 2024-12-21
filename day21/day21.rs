use std::collections::HashMap;

use aoc2024::{file, l2d::{grid::Grid, position::Position}};

// ---

// Conclusion - hash:en funkar
const USE_HASH: bool = true;
const ITERATIONS: u64 = 25;

fn main() {
    
    // After one dpad-sequence we get this: <A>A<AAv<AA>>^AvAA^Av<AAA>^A
    // It is present in the brute-force solution. 

//     println!("{}", get_numpad_sequence("379A"));
//     panic!();

//     assert_contains(get_directional_pad_sequence(&get_numpad_sequence("379A")),
//     vec!(
//        "<A>A<AAv<AA>>^AvAA^Av<AAA^>A", "<A>A<AAv<AA>>^AvAA^Av<AAA>^A", "<A>A<AAv<AA>>^AvAA^A<vAAA^>A", "<A>A<AAv<AA>>^AvAA^A<vAAA>^A", "<A>Av<<AA>^AA>AvAA^Av<AAA^>A", "<A>Av<<AA>^AA>AvAA^Av<AAA>^A", "<A>Av<<AA>^AA>AvAA^A<vAAA^>A", "<A>Av<<AA>^AA>AvAA^A<vAAA>^A"
//     )
//    );

//     assert_contains(get_directional_pad_sequence(&get_directional_pad_sequence(&get_numpad_sequence("379A"))),
//      vec!(
//         "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>Av<A<A>>^AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>Av<A<A>>^AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>A<vA<A>>^AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>A<vA<A>>^AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>Av<A<A>>^AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>Av<A<A>>^AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>A<vA<A>>^AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>A<vA<A>>^AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>Av<A<A>>^AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>Av<A<A>>^AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>A<vA<A>>^AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>A<vA<A>>^AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>Av<A<A>>^AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>Av<A<A>>^AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>A<vA<A>>^AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>A<vA<A>>^AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>Av<A<A>>^AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>Av<A<A>>^AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>A<vA<A>>^AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>A<vA<A>>^AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>Av<A<A>>^AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>Av<A<A>>^AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>A<vA<A>>^AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>A<vA<A>>^AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>Av<A<A>>^AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>Av<A<A>>^AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>A<vA<A>>^AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>A<vA<A>>^AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<A<A>>^AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<A<A>>^AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<vA<A>>^AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<vA<A>>^AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>Av<A<A>>^AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>Av<A<A>>^AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>A<vA<A>>^AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>A<vA<A>>^AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>Av<A<A>>^AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>Av<A<A>>^AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>A<vA<A>>^AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>A<vA<A>>^AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>Av<A<A>>^AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>Av<A<A>>^AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>A<vA<A>>^AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>A<vA<A>>^AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>Av<A<A>>^AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>Av<A<A>>^AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>A<vA<A>>^AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>A<vA<A>>^AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>Av<A<A>>^AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>Av<A<A>>^AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>A<vA<A>>^AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>A<vA<A>>^AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>Av<A<A>>^AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>Av<A<A>>^AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>A<vA<A>>^AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>A<vA<A>>^AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>Av<A<A>>^AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>Av<A<A>>^AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>A<vA<A>>^AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>A<vA<A>>^AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<A<A>>^AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<A<A>>^AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<vA<A>>^AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<vA<A>>^AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>Av<A<A>>^AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>Av<A<A>>^AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>A<vA<A>>^AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>A<vA<A>>^AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>Av<A<A>>^AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>Av<A<A>>^AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>A<vA<A>>^AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>A<vA<A>>^AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>Av<A<A>>^AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>Av<A<A>>^AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>A<vA<A>>^AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>A<vA<A>>^AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>Av<A<A>>^AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>Av<A<A>>^AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>A<vA<A>>^AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>A<vA<A>>^AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>Av<A<A>>^AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>Av<A<A>>^AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>A<vA<A>>^AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>A<vA<A>>^AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>Av<A<A>>^AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>Av<A<A>>^AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>A<vA<A>>^AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>A<vA<A>>^AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>Av<A<A>>^AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>Av<A<A>>^AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>A<vA<A>>^AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>A<vA<A>>^AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<A<A>>^AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<A<A>>^AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<vA<A>>^AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<vA<A>>^AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>Av<A<A>>^AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>Av<A<A>>^AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>A<vA<A>>^AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>A<vA<A>>^AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>Av<A<A>>^AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>Av<A<A>>^AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>A<vA<A>>^AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>A<vA<A>>^AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>Av<A<A>>^AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>Av<A<A>>^AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>A<vA<A>>^AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>A<vA<A>>^AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>Av<A<A>>^AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>Av<A<A>>^AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>A<vA<A>>^AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>A<vA<A>>^AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>Av<A<A>>^AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>Av<A<A>>^AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>A<vA<A>>^AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>A<vA<A>>^AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>Av<A<A>>^AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>Av<A<A>>^AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>A<vA<A>>^AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>A<vA<A>>^AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>Av<A<A>>^AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>Av<A<A>>^AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>A<vA<A>>^AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>A<vA<A>>^AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<A<A>>^AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<A<A>>^AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<vA<A>>^AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<vA<A>>^AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>Av<<A>A^>AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>Av<<A>A^>AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>Av<<A>A>^AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>Av<<A>A>^AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>Av<<A>A^>AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>Av<<A>A^>AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>Av<<A>A>^AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>Av<<A>A>^AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>Av<<A>A^>AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>Av<<A>A^>AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>Av<<A>A>^AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>Av<<A>A>^AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>Av<<A>A^>AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>Av<<A>A^>AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>Av<<A>A>^AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>Av<<A>A>^AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>Av<<A>A^>AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>Av<<A>A^>AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>Av<<A>A>^AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>Av<<A>A>^AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>Av<<A>A^>AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>Av<<A>A^>AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>Av<<A>A>^AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>Av<<A>A>^AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>Av<<A>A^>AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>Av<<A>A^>AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>Av<<A>A>^AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>Av<<A>A>^AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A^>AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A^>AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A>^AAA<Av>A^A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A>^AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>Av<<A>A^>AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>Av<<A>A^>AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>Av<<A>A>^AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>Av<<A>A>^AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>Av<<A>A^>AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>Av<<A>A^>AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>Av<<A>A>^AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>Av<<A>A>^AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>Av<<A>A^>AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>Av<<A>A^>AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>Av<<A>A>^AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>Av<<A>A>^AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>Av<<A>A^>AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>Av<<A>A^>AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>Av<<A>A>^AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>Av<<A>A>^AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>Av<<A>A^>AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>Av<<A>A^>AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>Av<<A>A>^AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>Av<<A>A>^AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>Av<<A>A^>AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>Av<<A>A^>AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>Av<<A>A>^AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>Av<<A>A>^AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>Av<<A>A^>AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>Av<<A>A^>AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>Av<<A>A>^AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>Av<<A>A>^AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A^>AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A^>AAA<A>vA^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A>^AAA<Av>A^A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A>^AAA<A>vA^A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>Av<<A>A^>AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>Av<<A>A^>AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>Av<<A>A>^AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>Av<<A>A>^AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>Av<<A>A^>AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>Av<<A>A^>AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>Av<<A>A>^AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>Av<<A>A>^AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>Av<<A>A^>AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>Av<<A>A^>AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>Av<<A>A>^AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>Av<<A>A>^AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>Av<<A>A^>AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>Av<<A>A^>AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>Av<<A>A^>AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>Av<<A>A^>AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>Av<<A>A>^AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>Av<<A>A>^AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>Av<<A>A^>AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>Av<<A>A^>AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>Av<<A>A>^AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>Av<<A>A>^AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>Av<<A>A^>AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>Av<<A>A^>AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>Av<<A>A>^AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>Av<<A>A>^AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A^>AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A^>AAAvA<^A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA^<A>A", "v<<A>>^AvA^Av<A<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>Av<<A>A^>AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>Av<<A>A^>AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>Av<<A>A>^AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>Av<<A>A>^AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>Av<<A>A^>AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>Av<<A>A^>AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>Av<<A>A>^AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^Av<A>^AA<A>Av<<A>A>^AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>Av<<A>A^>AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>Av<<A>A^>AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>Av<<A>A>^AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA^>AA<A>Av<<A>A>^AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>Av<<A>A^>AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>Av<<A>A^>AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA^<A>AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>Av<<A>A^>AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>Av<<A>A^>AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>Av<<A>A>^AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A^>AA<A>Av<<A>A>^AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>Av<<A>A^>AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>Av<<A>A^>AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>Av<<A>A>^AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^Av<A>^AA<A>Av<<A>A>^AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>Av<<A>A^>AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>Av<<A>A^>AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>Av<<A>A>^AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA^>AA<A>Av<<A>A>^AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A^>AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A^>AAAvA<^A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA^<A>A", "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A"
//      )
//     );
//     println!("v<<A>>^AvA^Av<A<AA>>^AAvA^<A>AAvA^Av<A^>AA<A>Av<A<A>>^AAA<Av>A^A");
//     panic!("that is all");

    let lines = file::lines_from_file("input.txt");
    // let lines = vec!("379A");

    let res2: Vec<(u64, u64)> = lines.into_iter().map(|l| {
        let ss = get_numpad_sequence(&l);
        //let ss = "<A>Av<<AA>^AA>AvAA^Av<AAA^>A";
        // for i in 0..25 {
        //     ss = get_directional_pad_sequence(&ss);
        //     println!("{} of 25; {} chars", ss.len(), i+1);
        // }
        let mut m: HashMap<(String, u64), u64> = HashMap::new();
        // let (cost, strang) = get_directional_pad_cost(&ss, 'A', 2, &mut m);
        let cost = get_directional_pad_cost(&ss, 'A', ITERATIONS, &mut m);
        // println!("Val: {}", strang);
        let nums = &l[0..l.len()-1];
        let num: u64 = nums.parse().unwrap();
        return (cost, num);
    }).collect();

    //let res2: Vec<(u64, u64)> = res.into_iter().map(|(shortest, num)| (shortest.len().try_into().unwrap(), num)).collect();
    println!("{:?}", res2);
    let sum: u64 = res2.into_iter().map(|(s, x)| s*x).sum();

    // 9355882408 Too low
    // 229023290884796 Too high. Joy.
    // 90525154388454 Too low (so not 24, off-by-one)
    // 154115708116294 "Not right"
    // 223902935165512 RIGHT - especially helpful to use "input.txt" instead of "text.txt"
    println!("{}",sum);

    // assert_numpad("379A", vec!["^A<<^^A>>AvvvA"]);
    // assert_dirpad("<^", vec!["v<<A>^A"]);

    // get_directional_pad_sequence(&get_numpad_sequence("029A"));

    // println!("Answer: {}", get_directional_pad_sequence("<A^A>^^AvvvA") == "v<<A>>^A<A>AvA<^AA>A<vAAA>^A");
    // println!("Answer: {}", get_directional_pad_sequence("<A^A^>^AvvvA") == "v<<A>>^A<A>AvA<^AA>A<vAAA>^A");
    // println!("Answer: {}", get_directional_pad_sequence("<A^A^^>AvvvA") == "v<<A>>^A<A>AvA<^AA>A<vAAA>^A");
}

fn assert_contains(result: String, actual_ok: Vec<&str>) {
    let m: Vec<String> = actual_ok.iter().filter(|&&s| s == result).map(|&s| s.to_owned()).collect();
    println!("Expected {}", result);
    println!("{:?}", m);
}

fn get_numpad_sequence(input: &str) -> String {
    let numpad = Grid::new(vec![vec!['7','8','9'],vec!['4','5','6'],vec!['1','2','3'],vec![' ','0','A']]);
    let mut curr_pos = Position { x: 2, y: 3 };
    let mut s = String::new();

    let mut curr = 0;

    while curr < input.len() {
        let target_pos = numpad.find_first(input.chars().nth(curr).unwrap()).unwrap();
        loop {
            let diff = target_pos - curr_pos;
            let down_ok = curr_pos.x != 0 || target_pos.y != 3;
            let left_ok = curr_pos.y != 3 || target_pos.x != 0;

            if left_ok && diff.x < 0 {
                // Go left
                let steps = diff.x.abs() as usize;
                s += &"<".repeat(steps);
                curr_pos.x += diff.x;
            } else if down_ok && diff.y > 0 {
                // Go down
                let steps = diff.y.abs() as usize;
                s += &"v".repeat(steps);
                curr_pos.y += diff.y;
            } else if diff.y < 0 {
                // Go up
                let steps = diff.y.abs() as usize;
                s += &"^".repeat(steps);
                curr_pos.y += diff.y;
            } else if diff.x > 0 {
                // Go right
                let steps = diff.x.abs() as usize;
                s += &">".repeat(steps);
                curr_pos.x += diff.x;
            } else {
                // Click A!
                s += "A";
                curr += 1;
                break;
            }
        }
    }
    s
}

// fn get_directional_pad_cost(input: &str, mut prev: char, steps: u64, hash: &mut HashMap<(char, char, u64), u64>) -> (u64, String) {
//     if steps == 0 {
//         return (input.len().try_into().unwrap(), input.to_string());
//     }
//     let mut cost = 0;
//     let mut strang = String::new();
//     for c in input.chars() {
//         // if let Some(cost) = hash.get(&(prev, c, steps)) {
//         //     return *cost;
//         // }
//         let (cst, s) = get_directional_pad_cost(&char_to_next_step_instructions(prev, c), 'A', steps-1, hash);
//         cost += cst;
//         strang += &s;
//         prev = c;
//     }
//     (cost, strang)
// }

fn get_directional_pad_cost(input: &str, mut prev: char, steps: u64, hash: &mut HashMap<(String, u64), u64>) -> u64 {
    if steps == 0 {
        return input.len().try_into().unwrap();
    }
    if USE_HASH {
        if let Some(cost) = hash.get(&(input.to_owned(),steps)) {
            return *cost;
        }
    }
    let mut cost = 0;
    for c in input.chars() {
        let cst = get_directional_pad_cost(&char_to_next_step_instructions(prev, c), 'A', steps-1, hash);
        cost += cst;
        prev = c;
    }
    if USE_HASH {
        hash.insert((input.to_owned(), steps), cost);
    }
    cost
}

fn char_to_next_step_instructions(prev_c: char, c: char) -> String {
    /*let s= */match prev_c {
        '>' => 
        match c {
            '>' => "A",
            '^' => "<^A",
            '<' => "<<A",
            'v' => "<A",
            'A' => "^A",
            _ => panic!("Unexpected char {}", c),
        },
        '<' => 
        match c {
            '>' => ">>A",
            '^' => ">^A",
            '<' => "A",
            'v' => ">A",
            'A' => ">>^A",
            _ => panic!("Unexpected char {}", c),
        },
        'v' => 
        match c {
            '>' => ">A",
            '^' => "^A",
            '<' => "<A",
            'v' => "A",
            'A' => "^>A",
            _ => panic!("Unexpected char {}", c),
        },
        '^' => 
        match c {
            '>' => "v>A",
            '^' => "A",
            '<' => "v<A",
            'v' => "vA",
            'A' => ">A",
            _ => panic!("Unexpected char {}", c),
        },
        'A' => 
        match c {
            '>' => "vA",
            '^' => "<A",
            '<' => "v<<A",
            'v' => "<vA",
            'A' => "A",
            _ => panic!("Unexpected char {}", c),
        },
        _ => panic!("Unexpected previous char {}", prev_c)
    }.to_string()/*;
    println!("{}->{}: {}", prev_c, c, s);
    s*/
}

fn get_directional_pad_sequence(input: &str) -> String {
    // let priority = vec![Direction::Right, Direction::Up, Direction::Down, Direction::Left];

    let numpad = Grid::new(vec![vec![' ','^','A'],vec!['<','v','>']]);
    let mut curr_pos = Position { x: 2, y: 0 };
    let mut s = String::new();

    let mut curr = 0;

    while curr < input.len() {
        let target_pos = numpad.find_first(input.chars().nth(curr).unwrap()).unwrap();
        loop {
            let diff = target_pos - curr_pos;
            if diff.x > 0 {
                // Go right
                s += ">";
                curr_pos = curr_pos.right();
            } else if diff.y < 0 {
                // Go up
                s += "^";
                curr_pos = curr_pos.up();
            } else if diff.y > 0 {
                // Go down
                s += "v";
                curr_pos = curr_pos.down();

            } else if diff.x < 0 {
                // Go left
                s += "<";
                curr_pos = curr_pos.left();
            } else {
                // Click A!
                s += "A";
                break;
            }
        }
        curr += 1;
    }
    s
}

fn assert_numpad(input: &str, result: Vec<&str>) {
    let rs = get_numpad_sequence(input);
    println!("{}", rs);
    assert!(result.into_iter().find(|&p| p == rs).is_some());
}

fn assert_dirpad(input: &str, result: Vec<&str>) {
    let rs = get_directional_pad_sequence(input);
    println!("{}", rs);
    assert!(result.into_iter().find(|&p| p == rs).is_some());
}