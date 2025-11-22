#![allow(unused)]

use env_logger;
use itertools::Itertools;
use log::{debug, info, trace};
use std::collections::HashSet;

fn parse_weights(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

pub fn part_1() {
    env_logger::init();
    let input = include_str!("inputs/day24.txt");
    let weights = parse_weights(input);

    // We want the same amount of weight in all three groups
    // Go through all possible combinations of groups?
    // (Or) mathematically deduce which groups would yield right results

    // Let's try combinations first
    // Okay, let's go through all the shuffles and group them correctly
    let mut valid_combs: HashSet<Vec<Vec<i32>>> = HashSet::new(); // ugh, nesting
    for group in weights.iter().permutations(weights.len()) {
        trace!("{group:?}");
        for mut index_separators in (1..weights.len()-1).combinations(2) {
            index_separators.sort();
            let low = index_separators[0];
            let high = index_separators[1];
            let group1: Vec<i32> = group[..low].iter().copied().collect();
            let group2: Vec<i32> = group[low..high].iter().copied().collect();
            let group3: Vec<i32> = group[high..].iter().copied().collect();

            let s1 = group1.iter().copied().sum::<i32>();
            let s2 = group2.iter().copied().sum::<i32>();
            let s3 = group3.iter().copied().sum::<i32>();
            
            if s1 == s2 && s2 == s3 {
                valid_combs.insert(vec![group1, group2, group3]);
            }
        }
    }

    for comb in valid_combs {
        println!("{comb:?}");
    }
}
