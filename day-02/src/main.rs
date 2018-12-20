use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::vec::Vec;
use std::collections::HashMap;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    filename: String,
}

fn parse(file: File) -> Vec<String> {
    Vec::from_iter(BufReader::new(file).lines().map(|l| l.unwrap()))
}

fn count(id: &String) -> (i32, i32) {
    let mut counter = HashMap::new();
    for c in id.chars() {
        *counter.entry(c).or_insert(0) += 1;
    }
    return (counter.values().any(|n| *n == 2) as i32, counter.values().any(|n| *n == 3) as i32)
}

fn part_1(ids: &Vec<String>) -> i32 {
    let (n, m) = ids.iter().map(count).fold((0,0), |a, b| (a.0 + b.0, a.1 + b.1));
    n * m
}

fn levenshtein(a: &String, b: &String) -> i32 {
    a.chars().zip(b.chars()).fold(0, |acc, x| acc + ((x.0 != x.1) as i32))
}

fn matching_chars(a: &String, b: &String) -> String {
    a.chars().zip(b.chars()).filter(|x| x.0 == x.1).map(|x| x.0).collect::<String>()
}

fn part_2(ids: &Vec<String>) -> Result<String, String> {
    for i in 0..ids.len() {
        for j in (i+1)..ids.len() {
            if levenshtein(&ids[i], &ids[j]) == 1 {
                return Ok(matching_chars(&ids[i], &ids[j]));
            }
        }
    }
    Err("no match found".to_string())
}

fn main() {
    let args = Cli::from_args();
    let file = File::open(&args.filename).expect(&format!("cannot open \"{}\"", args.filename));
    let ids = parse(file);
    println!("{}", part_1(&ids)); 
    match part_2(&ids) {
        Ok(s) => println!("Match: {:?}", s),
        Err(e) => println!("{:?}", e)
    }
}
