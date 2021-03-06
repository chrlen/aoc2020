use std::io::*;
use std::fs::File;
use std::vec::*;

fn main(){
    let filename: &str = "data/2.1.csv";
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);

    let vec_a: Vec<i32>  = f.lines()
    .map(|x| x.unwrap())
    .map(|x| {
        //x.parse::<i32>().unwrap()
        let s0 = x.split(": ").collect::<Vec<&str>>();
        let f = s0[0];
        let s1 = f.split(" ").collect::<Vec<&str>>();
        let range = s1[0];
        let s2 = range.split("-").collect::<Vec<&str>>();

        let pw: Vec<char> = s0[1].chars().collect();
        let c = s1[1].chars().next().unwrap();
        let l = s2[0].parse::<usize>().unwrap() - 1 ;
        let u = s2[1].parse::<usize>().unwrap() - 1 ;
        
        let arr : Vec<i32> = vec!(
            (pw[l] == c) as i32,
           ( pw[u] == c) as i32
        );


        let res: bool = arr.iter().sum::<i32>() == 1;
        // let res = (pw[l] == c) ^ (pw[u] != c) || (pw[u] == c) ^ (pw[l] != c);
        //let count = pw.chars().filter(|symb| *symb == c).count();
        //let res = l <= count as i32 && count as i32 <= u;
        res as i32

    }).collect();

    for i in vec_a.iter(){
        println!("{}",i);
    }
    println!("{}",vec_a.iter().sum::<i32>());
    // let res: Vec<i32> = vec_a.clone().into_iter()
    // .cartesian_product(vec_a)
    // .filter(|(a,b)| a+b == 2020)
    // .map(|(a,b)| a*b)
    // .collect();
    //let res = vec_a.iter().zip(vec_a.iter()).collect::<Vec<(i32,i32)>>();


    // println!("{}",res[0]);

    }