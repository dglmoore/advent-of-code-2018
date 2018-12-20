use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::vec::Vec;
use std::collections::HashSet;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    filename: String,
}

fn parse(file: File) -> Vec<i32> {
    let iter = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap());

    Vec::from_iter(iter)
}

fn part_1(changes: &Vec<i32>) -> i32 {
    changes.iter().sum::<i32>()
}

fn part_2(changes: &Vec<i32>) -> i32 {
    let mut frequencies = HashSet::new();
    let mut frequency = 0;
    for change in changes.iter().cycle() {
        if frequencies.contains(&frequency) {
            break
        }
        frequencies.insert(frequency);
        frequency += change;
    }
    frequency
}

fn main() {
    let args = Cli::from_args();
    let file = File::open(&args.filename).expect(&format!("cannot open \"{}\"", args.filename));
    let changes = parse(file);
    println!("{}", part_1(&changes));
    println!("{}", part_2(&changes));
}
