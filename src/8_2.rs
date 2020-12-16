use std::collections::HashSet;
use std::fs::File;
use std::io::*;
use std::vec::*;

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
        // println!("{} {}", op, step);
        match &op[..] {
            "nop" => (index + 1, acc),
            "acc" => (index + 1, acc + step),
            "jmp" => (index + step, acc),
            &_ => (index, acc),
        }
    }

    fn terminates(p: &Vec<(i32, String, i32)>) -> (bool, i32) {
        let mut done = false;
        let mut index = 0;
        let mut acc = 0;
        let mut mem: HashSet<i32> = HashSet::new();
        let mut res = 0;

        while !done {
            let (new_index, new_acc) = iterate(index, acc, p);
            // println!("{} {}", new_index, new_acc);

            if mem.contains(&new_index) {
                res = acc;
                done = true;
                return (false, acc);
            } else if new_index == p.len() as i32 {
                return (true, new_acc);
            } else {
                mem.insert(new_index);
                index = new_index;
                acc = new_acc;
            }
        }
        (true, 4)
    }
    let (res, acc) = terminates(&program);
    // println!("{} {}", res, acc);
    let p_mod: Vec<Vec<(i32, String, i32)>> = program
        .iter()
        .filter(|(num, op, step)| *op == "nop" || *op == "jmp")
        .map(|(num, op, step)| {
            let new: Vec<(i32, String, i32)> = program
                .iter()
                .map(|(n, o, s)| {
                    if n == num {
                        if op == "nop" {
                            //(n, "jmp", s)
                            (n.clone(), "jmp".to_string(), s.clone())
                        } else {
                            //(n, "nop", s)
                            (n.clone(), "nop".to_string(), s.clone())
                        }
                    } else {
                        (n.clone(), o.to_string(), s.clone())
                    }
                })
                .collect();
            new
        })
        .collect();

    let res: Vec<(bool, i32)> = p_mod
        .iter()
        .map(|x| terminates(&x))
        .filter(|(a, b)| *a)
        .collect();

    for (res, acc) in res {
        println!("{} {}", res, acc);
    }
}
