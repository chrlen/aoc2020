use std::fs::File;
use std::io::*;
use std::vec::*;

fn decode(code: &str) -> i32 {
    fn transform(c: char) -> char {
        if c == 'F' {
            return '0';
        } else if c == 'B' {
            return '1';
        } else if c == 'L' {
            return '0';
        } else {
            return '1';
        }
    }

    let bin: String = code.chars().map(transform).collect();
    let res: i32 = isize::from_str_radix(&bin, 2).unwrap() as i32;
    //println!("code: {} res: {}", code, res);
    res
}

fn main() {
    let filename: &str = "data/5.csv";
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    let lines: Vec<String> = f.lines().map(|x| x.unwrap()).collect();

    let row_code_length = 7;
    let col_code_length = 3;

    let split: Vec<(i32, i32)> = lines
        .iter()
        .map(|line| {
            (
                decode(&line[row_code_length..]),
                decode(&line[..row_code_length]),
            )
        })
        .collect();
    let mut seat_ids = split.iter().map(|x| x.1 * 8 + x.0).collect::<Vec<i32>>();

    seat_ids.sort();
    let res_windows: Vec<Vec<i32>> = seat_ids
        .windows(3)
        .filter(|s| (s[0] + 1 == s[1]) ^ (s[1] + 1 == s[2]))
        .map(|x| x.to_vec())
        .collect();

    for i in res_windows{
        println!("{} {} {}",i[0],i[1],i[2]);
    }
}
