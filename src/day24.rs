#![allow(unused)]

use env_logger;
use itertools::Itertools;
use log::{debug, info, trace};
use std::collections::HashSet;

fn parse_weights(input: &str) -> Vec<i128> {
    input
        .lines()
        .map(|line| line.parse::<i128>().unwrap())
        .collect()
}

pub fn part_1() {
    env_logger::init();
    let input = include_str!("inputs/day24.txt");
    let weights = parse_weights(input);
    let target = weights.iter().sum::<i128>() / 3;
    let mut qe = i128::MAX;
    let mut minlen = usize::MAX;
    let mut done = false;

    for i in 0..weights.len() {
        for comb in weights.iter().combinations(i) {
            let len = comb.len();
            let sum = comb.iter().copied().sum::<i128>();

            if sum == target {
                if len < minlen {
                    minlen = len;
                    qe = comb.into_iter().product::<i128>();
                    done = true;
                }
            }
        }

        if done {
            break;
        }
    }

    info!("{qe}");
}

pub fn part_2() {
    env_logger::init();
    let input = include_str!("inputs/day24.txt");
    let weights = parse_weights(input);
    let target = weights.iter().sum::<i128>() / 4;
    let mut qe = i128::MAX;
    let mut minlen = usize::MAX;
    let mut done = false;

    for i in 0..weights.len() {
        for comb in weights.iter().combinations(i) {
            let len = comb.len();
            let sum = comb.iter().copied().sum::<i128>();

            if sum == target {
                if len < minlen {
                    minlen = len;
                    qe = comb.into_iter().product::<i128>();
                    done = true;
                }
            }
        }

        if done {
            break;
        }
    }

    info!("{qe}");
}
