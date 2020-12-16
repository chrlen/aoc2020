use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::io::*;
use std::vec::*;
use utils::*;

#[macro_use]
extern crate lazy_static;

fn main() {
    let filename: &str = "data/010.csv";
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    let mut lines: Vec<i32> = f
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    lines.push(0);
    lines.sort();
    let mut end = vec![lines[lines.len() - 1] + 3];
    lines.append(&mut end);

    for l in lines.clone() {
        //    println!("{}", l);
    }

    for i in 0..(lines.len() - 1) {
        lines[i] = lines[i + 1] - lines[i];
        // println!("{}", lines[i]);
    }
    let ones = lines.iter().filter(|x| **x == 1).count();
    let threes = lines.iter().filter(|x| **x == 3).count();
    println!("{}", ones);
    println!("{}", threes);
    println!("{}", ones * threes);
}
