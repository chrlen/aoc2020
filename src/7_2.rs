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
    fn count(name: String, map: &HashMap<String, Vec<(String, i32)>>) -> i32 {
        let content: &Vec<(String, i32)> = &map[&name];
        if content.is_empty() {
            0
        } else {
            content
                .iter()
                .map(|(name, cap)| cap + cap * count(name.clone(), map))
                .sum::<i32>()
        }
    };
    println!("{}", count("shiny gold".to_string(), &s));

    // for (c, v) in s {
    //     println!(" ");
    //     println!("{}", c);
    //     for (c, b) in &v {
    //         println!("{} {}", c, b);
    //     }
    // }
}
