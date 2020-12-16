use std::fs::File;
use std::io::*;
use std::vec::*;
use utils::*;

fn main() {
    let filename: &str = "data/010_test.csv";
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    let mut lines: Vec<i64> = f
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    lines.push(0);
    lines.sort();
    let mut end = vec![lines[lines.len() - 1] + 3];
    lines.append(&mut end);

    let res: i64 = lines
        .windows(4)
        .enumerate()
        .map(|(i, s)| {
            if i % 3 == 0 {
                let a = s[0];
                let b = s[1];
                let c = s[2];
                let d = s[3];
                // b can be left out
                println!("{}: {} {} {} {}", i, a, b, c, d);
                if d - a <= 3 {
                    3
                } else if c - a <= 3 {
                    2
                } else {
                    1
                }
            } else {
                1
            }
        })
        .product();
    println!("{}", res);
}
