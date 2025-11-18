use itertools::Itertools;
use std::fs;

fn read_input(file_path: String) -> Vec<u32> {
    let mut containers: Vec<u32> = Vec::new();

    for line in fs::read_to_string(file_path)
        .expect("Unable to read file")
        .lines()
    {
        containers.push(line.parse::<u32>().expect("NaN"))
    }

    return containers;
}

pub fn part_1() {
    let containers = read_input("src/inputs/day17.txt".to_string());
    let target = 150;

    let mut answer = 0;

    for comb in containers.iter().powerset() {
        let sum: u32 = comb.iter().copied().sum();
        if sum == target {
            answer += 1;
        }
    }

    println!("{}", answer);
}

pub fn part_2() {
    let containers = read_input("src/inputs/day17.txt".to_string());
    let target = 150;

    let mut min = 1000;

    for comb in containers.iter().powerset() {
        let sum: u32 = comb.iter().copied().sum();
        if sum == target {
            let len = comb.iter().len();
            if len < min {
                min = len;
            }
        }
    }

    let mut answer = 0;
    for comb in containers.iter().powerset() {
        let sum: u32 = comb.iter().copied().sum();
        if sum == target {
            let len = comb.iter().len();
            if len == min {
                answer += 1;
            }
        }
    }

    println!("{}", answer);
}
