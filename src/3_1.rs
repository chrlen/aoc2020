use std::io::*;
use rayon::prelude::*;
use std::fs::File;
use std::vec::*;
use utils::*;
use std::any::type_name;
use itertools::Itertools;

fn main(){
    let filename: &str = "data/3.1.csv";
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    let vec_a: Vec<Vec<char>>  = f.lines()
    .map(|x| x.unwrap())
    .map(|x| x.chars().collect()).collect();

    let x_step = 3;
    let y_step = 1;

    let y_max = vec_a.len();
    let x_max = vec_a[0].len() * ((y_max / y_step) * x_step);

    println!("x_size {}",x_max);
    println!("y_size {}",y_max);

    let y_steps = (0..y_max).step_by(y_step);
    let x_steps: Vec<usize> = (0..x_max).step_by(x_step).map(|x| x % vec_a[0].len()).collect();

    // for y in x_steps.iter(){
    //     println!("{}",y);
    // }

    let steps = y_steps.zip(x_steps);
    let path: Vec<char> = steps.map(|(x,y)| vec_a[x][y]).collect();
    let res = path.iter().filter(|c| **c == '#').count();
    println!("{}",res);

    }
