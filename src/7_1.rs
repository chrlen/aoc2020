use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::*;
use std::vec::*;
use utils::*;

#[macro_use]
extern crate lazy_static;

fn main() {
    let filename: &str = "data/7.csv";
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    let lines = f.lines().map(|x| x.unwrap()).collect::<Vec<String>>();

    lazy_static! {
        static ref re_content: Regex =
            Regex::new("(?P<capacity>[0-9]+) (?P<contained>[a-z ]+) bag").unwrap();
        static ref re_container: Regex =
            Regex::new("(?P<container>[a-z]+ [a-z]+) bags contain").unwrap();
    }

    let s: HashMap<String, Vec<(String, i32)>> = lines
        .iter()
        .cloned()
        .map(|line| {
            //println!("{}", line);
            let container = re_container
                .captures(&line)
                .and_then(|cap| cap.name("container").map(|con| con.as_str()));

            let contents: Vec<(String, i32)> = re_content
                .captures_iter(&line)
                .map(|cap| {
                    (
                        cap.name("contained").unwrap().as_str().to_owned(),
                        cap.name("capacity")
                            .unwrap()
                            .as_str()
                            .parse::<i32>()
                            .unwrap(),
                    )
                })
                .collect();
            (container.unwrap().to_owned(), contents)
        })
        .collect();
    let gold: Vec<i32> = s
        .iter()
        .filter(|(a, b)| !(*a == "shiny gold"))
        .map(|(container, content)| {
            let mut bags_to_check: VecDeque<String> = content
                .iter()
                .map(|(name, count)| name.to_string())
                .collect();

            while bags_to_check.len() > 0 {
                let target = bags_to_check.pop_front().unwrap();
                if target == "shiny gold" {
                    return 1;
                } else {
                    for (name, cap) in &s[&target] {
                        bags_to_check.push_back(name.clone());
                    }
                }
            }

            return 0;
        })
        .collect();
    let res: i32 = gold.iter().sum();
    println!("{}", res);

    // for (c, v) in s {
    //     println!(" ");
    //     println!("{}", c);
    //     for (c, b) in &v {
    //         println!("{} {}", c, b);
    //     }
    // }
}
