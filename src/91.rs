use itertools::Combinations;
use itertools::Itertools;
use itertools_num::linspace;
use std::fs::File;
use std::io::*;
use std::ops::Range;
use std::vec::*;

fn main() {
    let filename: &str = "data/9.csv";
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    let lines = f
        .lines()
        .map(|x| x.unwrap().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let preamble_lenght = 25;
    let indices: Range<usize> = (preamble_lenght)..lines.len();
    let ranges: Vec<(usize, usize)> = indices.map(|x| (x - preamble_lenght, x)).collect();

    let tasks: Vec<(i64, i64)> = ranges
        .iter()
        .map(|(first, last)| {
            let target: i64 = lines[*last];

            let pairs: Vec<i64> = (*first..(*last - 1))
                .combinations(2)
                .map(|v| lines[v[0] as usize] + lines[v[1] as usize])
                .filter(|x| *x == target)
                .collect();
            //println!("{} {}", target, pairs.len());
            (target, pairs.len() as i64)
        })
        .collect();

    let res: Vec<&(i64, i64)> = tasks.iter().filter(|(a, b)| *b == 0).collect();
    for (a, b) in res {
        println!("{} {}", a, b);
    }
}
