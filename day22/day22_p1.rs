use aoc2024::file;

// ---

fn main() {
    let lines = file::lines_from_file("input.txt");
    let init_nums: Vec<usize> = /*vec![123];*/ lines
                                           .into_iter()
                                           .map(|l| l.parse::<usize>().unwrap())
                                           .collect();

    let secrets: Vec<usize> = init_nums
        .into_iter()
        .map(|num| n_th_secret(num, 2000))
        .collect();

    println!("Secrets: {:?}", secrets);
    println!("Answer: {}", secrets.iter().sum::<usize>());
}

fn n_th_secret(first_secret: usize, n: usize) -> usize {
    let mut curr = first_secret;
    for _ in 0..n {
        let s1 = ((curr * 64) ^ curr) % 16777216;
        let s2 = ((s1 / 32) ^ s1) % 16777216;
        let s3 = ((s2 * 2048) ^ s2) % 16777216;
        curr = s3;
    }
    curr
}
