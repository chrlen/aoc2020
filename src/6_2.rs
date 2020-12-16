use std::collections::HashSet;
use std::fs::File;
use std::io::*;
use std::vec::*;
use utils::*;

fn main() {
    let filename: &str = "data/6.csv";
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    let vec_a = f.lines().map(|x| x.unwrap());

    let mut multiple_lines: Vec<Vec<String>> = vec![];
    let mut current: Vec<String> = Vec::new();

    for line in vec_a {
        if line.len() == 0 {
            multiple_lines.push(current);
            current = Vec::new();
        } else {
            current.push(line);
        }
    }

    let sets: Vec<Vec<HashSet<String>>> = multiple_lines
        .iter()
        .map(|x| {
            x.iter()
                .map(|y| {
                    let mut h: HashSet<String> = HashSet::new();
                    for c in y.chars() {
                        h.insert(c.to_string());
                    }
                    h
                })
                .collect()
        })
        .collect();

    let groups: Vec<HashSet<String>> = sets
        .iter()
        .map(|group| {
            let union: HashSet<String> = group.iter().fold(HashSet::new(), |a, b| {
                a.union(&b).cloned().collect::<HashSet<String>>()
            });
            union
                .clone()
                .iter()
                .filter(|elem| group.iter().all(|person| person.contains(elem.clone())))
                .cloned()
                .collect::<HashSet<String>>()
        })
        .collect();

    let res: i32 = groups.iter().map(|x| x.len() as i32).sum();
    println!("{}", res);
}
