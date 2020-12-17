use itertools::Itertools;
use rayon::prelude::*;
use std::any::type_name;
use std::fs::File;
use std::io::*;
use std::vec::*;
use utils::*;

fn main() {
    let filename: &str = "data/3.1.csv";
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    // Read input as a Vector of Char-Vectors
    let vec_a: Vec<Vec<char>> = f
        .lines()
        .map(|x| x.unwrap())
        .map(|x| x.chars().collect())
        .collect();

    // Set Stepwidth for x and y direction
    let x_step = 3;
    let y_step = 1;

    // Set upper bounds for movements in x and y direction
    let y_max = vec_a.len();
    let x_max = vec_a[0].len() * ((y_max / y_step) * x_step);

    println!("x_size {}", x_max);
    println!("y_size {}", y_max);

    // Calculate indices in y direction
    let y_steps = (0..y_max).step_by(y_step);

    // Calculate indices in x direction with modulus for board repitition
    let x_steps: Vec<usize> = (0..x_max)
        .step_by(x_step)
        .map(|x| x % vec_a[0].len())
        .collect();

    let steps = y_steps.zip(x_steps);

    // Select the path from the input
    let path: Vec<char> = steps.map(|(x, y)| vec_a[x][y]).collect();

    // Count occurences of the symbol '#' in the path
    let res = path.iter().filter(|c| **c == '#').count();
    println!("{}", res);
}
