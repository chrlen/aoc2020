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
    let filename: &str = "data/8.csv";
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    let lines = f
        .lines()
        .map(|x| x.unwrap())
        .enumerate()
        .collect::<Vec<(usize, String)>>();

    let program: Vec<(i32, String, i32)> = lines
        .iter()
        .map(|(a, b)| {
            let s: Vec<&str> = b.split(" ").collect();
            let op: String = s[0].to_string();
            let step = s[1].parse::<i32>().unwrap();
            (*a as i32, op, step)
        })
        .collect();

    fn iterate(index: i32, acc: i32, p: &Vec<(i32, String, i32)>) -> (i32, i32) {
        let (num, op, step) = p.get(index as usize).unwrap();
        println!("{} {}", op, step);
        match &op[..] {
            "nop" => (index + 1, acc),
            "acc" => (index + 1, acc + step),
            "jmp" => (index + step, acc),
            &_ => (index, acc),
        }
    }
    let mut done = false;
    let mut index = 0;
    let mut acc = 0;
    let mut mem: HashSet<i32> = HashSet::new();
    let mut res = 0;
    while !done {
        let (new_index, new_acc) = iterate(index, acc, &program);
        println!("{} {}", new_index, new_acc);
        if mem.contains(&new_index) {
            res = acc;
            done = true;
        } else {
            mem.insert(new_index);
            index = new_index;
            acc = new_acc;
        }
    }
    println!("res :{}", res);
}
