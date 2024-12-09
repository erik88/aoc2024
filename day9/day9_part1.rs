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

#[derive(PartialEq)]
enum Read {
    File,
    Space
}

fn main() {
    let lines = lines_from_file("input.txt");
    let input = lines.concat();
    let mut layout: Vec<Option<u64>> = Vec::new();
    let mut reading = Read::File;
    let mut file_count = 0;

    for c in input.chars() {
        let ci: u64 = c.to_digit(10).unwrap().into();
        let val = if reading == Read::File { reading = Read::Space; file_count += 1; Some((file_count-1).try_into().unwrap()) } else { reading = Read::File; None };

        let mut tmp: Vec<Option<u64>> = vec![
            val;
            ci.try_into().unwrap()
        ];
        layout.append(&mut tmp);
    }

    let mut x = 0;
    let mut y = layout.len()-1;

    while x < y {
        let itm = layout.get(x).unwrap().clone();
        if itm == None {
            layout.swap(x, y);
            loop {
                y -= 1;
                if *layout.get(y).unwrap() != None {
                    break;
                }
            }
        }
        x += 1;
    }

    let mut sum: u64 = 0;
    for (i, itm) in layout.iter().enumerate() {
        if let Some(val) = itm {
            let index: u64 = i.try_into().unwrap();
            sum +=  index * (*val);
        } else {
            break;
        }
    }

    // let s: String = layout.iter().map(|v| match v { None => '.', Some(val) => val.to_string().chars().next().unwrap()}).collect();

    println!("Part 1: {}", sum);
}