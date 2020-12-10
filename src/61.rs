use std::io::*;
use std::fs::File;
use std::vec::*;
use utils::*;
use std::collections::HashSet;

fn main(){
    let filename: &str = "data/6.csv";
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    let vec_a  = f.lines().map(|x| x.unwrap());

    let mut multiple_lines: Vec<Vec<String>> = vec!();
    let mut current: Vec<String> =Vec::new();

    for line in vec_a {
        if line.len() == 0 {
            multiple_lines.push(current);
            current = Vec::new();        
        }
        else{
            //println!("{}",line);
            current.push(line);
        }
    }

    let groups: Vec<HashSet<String>> = multiple_lines.iter().map(|x| {
        let mut res: HashSet<String> = HashSet::new();
        for line in x{
            for c in line.chars(){
                // println!("{}",c);
                res.insert(c.to_string());
            }
        }
        res
    }).collect();

    let res: usize= groups.iter().map(|x| x.len()).sum();
    println!("{}",res);
}
