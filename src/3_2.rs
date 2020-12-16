use std::io::*;
use rayon::prelude::*;
use std::fs::File;
use std::vec::*;
use utils::*;
use std::any::type_name;
use itertools::Itertools;

fn trees(vec_a:Vec<Vec<char>>, x_step: i32, y_step: i32) -> i32 {

    let y_max = vec_a.len() as i32;
    let x_max = vec_a[0].len() as i32 * ((y_max / y_step) * x_step);

    let y_steps = (0..y_max).step_by(y_step as usize);
    let x_steps: Vec<i32> = (0..x_max).step_by(x_step as usize).map(|x| x % vec_a[0].len() as i32).collect();

    let steps = y_steps.zip(x_steps);
    let path: Vec<char> = steps.map(|(x,y)| vec_a[x as usize][y as usize]).collect();
    let res = path.iter().filter(|c| **c == '#').count();
    res as i32
}

fn main(){
    let filename: &str = "data/3.1.csv";
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    let vec_a: Vec<Vec<char>>  = f.lines()
    .map(|x| x.unwrap())
    .map(|x| x.chars().collect()).collect();

    let res = vec!(
        trees(vec_a.clone(), 1, 1),
        trees(vec_a.clone(), 3, 1),
        trees(vec_a.clone(), 5, 1),
        trees(vec_a.clone(), 7, 1),
        trees(vec_a.clone(), 1, 2)
    ).iter().fold(1,|a,b| a * b);
    println!("{}",res);

    }
