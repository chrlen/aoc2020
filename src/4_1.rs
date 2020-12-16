use std::io::*;
use rayon::prelude::*;
use std::fs::File;
use std::vec::*;
use utils::*;
use std::any::type_name;
use itertools::Itertools;
use std::collections::LinkedList;
use std::collections::HashMap;

fn main(){
    let filename: &str = "data/4.1_test.csv";
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    let vec_a  = f.lines().map(|x| x.unwrap());

    // println!("Empty lines: {}",vec_a.filter(|x| x.len() == 0).count());

    let mut multiple_lines: Vec<Vec<String>> = vec!();
    let mut current: Vec<String> = vec!();
    let mut app: String = String::from(" ");

    for line in vec_a{
        if line.len() == 0{
            multiple_lines.push(current);
            current = vec!();
        }
        else{
            for s in line.split(" "){
                let s2: Vec<&str> = s.split(":").collect();
                app = String::from(s2[0]);   
                current.push(app);
            }
        }
    }

    println!("{}",multiple_lines.len());

    let ports: Vec<Vec<String>> = multiple_lines;

    for p in ports.iter().take(10){
        println!("------------------");
        for key in p.iter(){
            println!("{}",key);
        }
    }

    let valid = |port: Vec<String>| -> bool {
        if !port.iter().any(|i| i == "eyr"){
            return false;
        }
        if !port.iter().any(|i| i == "pid"){
            return false;
        }
        if !port.iter().any(|i| i == "hcl"){
            return false;
        }
        if !port.iter().any(|i| i == "byr"){
            return false;
        }
        if !port.iter().any(|i| i == "iyr"){
            return false;
        }
        if !port.iter().any(|i| i == "ecl"){
            return false;
        }
        if !port.iter().any(|i| i == "hgt"){
            return false;
        }

        return true;
    };

    let res = ports.iter().filter(|y| valid(y.to_vec())).count();
    println!("{}",res);
}
