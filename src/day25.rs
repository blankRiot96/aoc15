#![allow(unused)]

use num_bigint::BigInt;

pub fn part_1() {
    let input = include_str!("inputs/day25.txt");
    let (left, right) = input.split_once(", column").unwrap();
    let row = left.split(" ").last().unwrap().parse::<i32>().unwrap();
    let col = right.trim().split_once(".").unwrap().0.parse::<i32>().unwrap();
    let d = row + col - 2;
    let n = (d * (d + 1) / 2) + col;
    let curr = (0..n-1).fold(BigInt::from(20151125), |acc, x| { (acc * BigInt::from(252533)) % BigInt::from(33554393) });
    println!("{curr}");
}

pub fn part_2() {
    println!("We have fixed the weather machine! It starts snowing :D");
}
