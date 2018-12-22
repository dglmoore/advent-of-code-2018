use std::{char, str, num};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::vec::Vec;
use core::ops::{Index, IndexMut};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    filename: String
}

#[derive(Debug)]
struct Rectangle {
    left: i32,
    top: i32,
    width: i32,
    height: i32
}

#[derive(Debug)]
struct Matrix {
    rows: usize,
    columns: usize,
    data: Vec<i32>
}

impl Matrix {
    fn new(rows: usize, columns: usize) -> Matrix {
        Matrix {
            rows: rows,
            columns: columns,
            data: vec![0i32; rows * columns]
        }
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = i32;

    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (i, j) = index;
        return &self.data[j + i * self.columns]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    #[inline]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (i, j) = index;
        return &mut self.data[j + i * self.columns]
    }
}

impl str::FromStr for Rectangle {
    type Err = num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<&str> = s.split(|c| !char::is_numeric(c)).collect();
        let left = fields[4].parse::<i32>()?;
        let top = fields[5].parse::<i32>()?;
        let width = fields[7].parse::<i32>()?;
        let height = fields[8].parse::<i32>()?;
        Ok(Rectangle {
            left: left,
            top: top,
            width: width,
            height: height
        })
    }
}

type Fabric = Matrix;

fn tape_off(fabric: &mut Fabric, claim: &Rectangle) {
    let right = claim.left + claim.width;
    let bottom = claim.top + claim.height;

    for i in claim.top..bottom {
        for j in claim.left..right {
            fabric[(i as usize, j as usize)] += 1;
        }
    }
}

fn parse(file: File) -> Vec<Rectangle> {
    BufReader::new(&file)
        .lines()
        .map(|line| line.unwrap().parse::<Rectangle>().unwrap())
        .collect()
}

fn part_1(claims: &Vec<Rectangle>) -> (Fabric, i32) {
    let mut fabric = Matrix::new(1000, 1000);
    for claim in claims.iter() {
        tape_off(&mut fabric, &claim);
    }
    let overlapped = fabric.data.iter().fold(0, |a, x| a + ((x > &1) as i32));
    (fabric, overlapped)
}

fn no_overlap(fabric: &Fabric, claim: &Rectangle) -> bool {
    let right = claim.left + claim.width;
    let bottom = claim.top + claim.height;

    for i in claim.top..bottom {
        for j in claim.left..right {
            if fabric[(i as usize, j as usize)] != 1 {
                return false;
            }
        }
    }
    return true;
}

fn part_2(fabric: &Fabric, claims: &Vec<Rectangle>) -> Result<i32, String> {
    for (id, claim) in claims.iter().enumerate() {
        if no_overlap(fabric, claim) {
            return Ok((id + 1) as i32);
        }
    }
    Err("all claims overlap".to_string())
}

fn main() {
    let args = Cli::from_args();

    let file = File::open(&args.filename).expect(&format!("cannot open {}", args.filename));
    let claims = parse(file);

    let (fabric, overlapped) = part_1(&claims);
    println!("{}", overlapped);

    match part_2(&fabric, &claims) {
        Ok(id) => println!("{}", id),
        Err(e) => eprintln!("{}", e)
    }
}
