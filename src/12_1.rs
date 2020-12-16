use std::fs::File;
use std::io::*;
use std::vec::*;
use utils::*;

fn print_board(b: Vec<Vec<String>>) {
    for v in b {
        for s in v {
            print!("{}", s);
        }
        println!(" ");
    }
}

fn boards_equal(b1: Vec<Vec<String>>, b2: Vec<Vec<String>>) -> bool {
    true
}

fn occupied_seats(b: Vec<Vec<String>>) -> i32 {
    b.iter()
        .map(|x| x.iter().filter(|y| *y == "#").count() as i32)
        .sum::<i32>()
}

fn iterate_board(b: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut res = b.clone();
    let x_max = res.len();
    let y_max = res[0].len();

    res
}

fn main() {
    let filename: &str = "data/12_test.csv";
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    let lines: Vec<Vec<String>> = f
        .lines()
        .map(|x| x.unwrap().chars().map(|x| x.to_string()).collect())
        .collect();

    let mut done = false;
    let mut now = lines.clone();
    let mut last = lines.clone();

    while !done {
        now = iterate_board(last.clone());

        print_board(now.clone());

        if boards_equal(last.clone(), now.clone()) {
            done = true;
            println!("Done: {}", occupied_seats(now.clone()));
        }
        last = now.clone();
    }
}
