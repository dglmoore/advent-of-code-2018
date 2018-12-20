use std::fs::File;
use std::io::{BufRead, BufReader};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    filename: String,
}

fn sum_lines(file: File) -> i32 {
    BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .sum()
}

fn main() {
    let args = Cli::from_args();
    let file = File::open(&args.filename).expect(&format!("cannot open \"{}\"", args.filename));
    println!("{}", sum_lines(file));
}
