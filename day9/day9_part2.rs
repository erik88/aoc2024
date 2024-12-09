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

#[derive(PartialEq, Clone, Copy)]
enum Content {
    FileIndex(u64),
    Space
}

#[derive(Clone, Copy)]
struct Block {
    content: Content,
    size: u64
}


fn main() {
    let lines = lines_from_file("input.txt");
    let input = lines.concat();
    let mut layout: Vec<Block> = Vec::new();
    let mut reading_file = true;
    let mut file_count = 0;

    for c in input.chars() {
        let ci: u64 = c.to_digit(10).unwrap().into();
        let block = if reading_file {
            let b = Block {
                size: ci,
                content: Content::FileIndex(file_count)
            };
            file_count += 1;
            b
        } else {
            Block {
                size: ci,
                content: Content::Space
            }
         };
        reading_file = !reading_file;

        layout.push(block);
    }

    let mut curr = 0;

    loop {
        let curr_itm = layout.get(curr).unwrap();
        if curr_itm.content != Content::Space {
            curr += 1;
            continue;
        }
        // Now we know curr_itm is a space

        let max_size = curr_itm.size;

        // Find a file from end with at most max_size
        let mut end = layout.len();
        let mut found: Option<usize> = None;

        loop {
            if end <= curr {
                break;
            }
            end -= 1;
            let end_itm = layout.get(end).unwrap();
            match end_itm.content {
                Content::Space => (),
                Content::FileIndex(_) => if end_itm.size <= max_size {
                    found = Some(end);
                    break;
                }
            }
        }

        if let Some(pos) = found {
            let pos_itm = layout.get(pos).unwrap();
            let size_left = curr_itm.size - pos_itm.size;
            if size_left == 0 {
                layout.swap(curr, pos);
            } else {
                layout[curr].size = pos_itm.size;
                layout.swap(curr, pos);
                layout.insert(curr+1, Block { size: size_left, content: Content::Space });
            }
            
        }

        curr += 1;
        if curr >= layout.len() {
            break;
        }
    }

    // let s: Vec<String> = layout.iter().map(
    //     |b| match b.content {
    //          Content::Space => vec!['.'; b.size.try_into().unwrap()].into_iter().collect(),
    //          Content::FileIndex(id) => vec![char::from_digit(id.try_into().unwrap(), 10).unwrap(); b.size.try_into().unwrap()].into_iter().collect()
    //     }
    // ).collect();
    //    println!("{}", s.join(""));
    
    let mut pos = 0;
    let mut checksum = 0;
    for block in layout {
        match block.content {
            // id*pos + id*(pos+1) + id*(pos+2) + ... id*(pos+(size-1))
            // id(pos + (pos + 1) + (pos + 2) + ... + (pos + (size-1)))
            // id(pos*size + 1 + 2 + ... + (size-1))
            // id(pos*size + (size*(size-1))/2)
            Content::FileIndex(id) => checksum += id*(pos*block.size + block.size*(block.size-1)/2),
            Content::Space => ()
        }
        pos += block.size;
    }

    println!("{}", checksum);
}
