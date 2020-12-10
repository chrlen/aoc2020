use std::io::*;
use rayon::prelude::*;
use std::fs::File;
use std::vec::*;
use utils::*;
use std::any::type_name;
use itertools::Itertools;

fn main(){
    let filename: &str = "data/1.1.csv";
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    let vec_a : Vec<i32> = f.lines().map(|x| x.unwrap()).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let res: Vec<i32> = vec_a.clone().into_iter()
    .cartesian_product(vec_a.clone())
    .cartesian_product(vec_a.clone())
    .map(|((a,b),c)| (a,b,c))
    .filter(|(a,b,c)| a+b+c == 2020)
    .map(|(a,b,c)| a*b*c)
    .collect();
    //let res = vec_a.iter().zip(vec_a.iter()).collect::<Vec<(i32,i32)>>();


    println!("{}",res[0]);

    }