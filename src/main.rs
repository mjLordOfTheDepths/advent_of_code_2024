use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let total = number_file(path);

    println!("No wait... sorry... the actual to-to-to-tal distance is: {}", total);
}

fn number_file(file: &Path) -> i32 {
    let mut l: Vec<i32> = Vec::new();
    let mut r: Vec<i32> = Vec::new();
    let mut l_new: Vec<i32> = Vec::new();
    let mut total = 0;
    let mut total_2 = 0;
    let input = File::open(file).unwrap();
    let reader = io::BufReader::new(input);

    for line in reader.lines() {
        let line = line.unwrap();
        let numbers = line.split_whitespace();

        for (i, num) in numbers.enumerate() {
            if i % 2 == 1 {
                l.push(num.parse().unwrap());
            } else {
                r.push(num.parse().unwrap());
            }
        }
    }
    
    l.sort();
    r.sort();
    l_new = l.clone();

    for i in 0..l.len() {
        total += ((r[i] - l[i]) as i32).abs();
        let mut repition = 0;
        for j in 0..l.len() {
            if r[j] == l_new[i] {
                repition += 1;
            }
        }
        l_new[i] *= repition;
        total_2 += l_new[i];
    }

    println!("Hohoho ! The answer is: {}", total);
    
    total_2
}