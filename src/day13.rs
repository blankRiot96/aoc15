use itertools::Itertools;
use std::collections::HashMap;

pub fn part_1() {
    let input = include_str!("inputs/day13.txt");
    let mut happiness: HashMap<(&str, &str), i32> = HashMap::new();
    let mut people: Vec<&str> = Vec::new();

    for line in input.lines() {
        let line = &line[..line.len() - 1];
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let pair = (tokens[0], tokens[10]);
        let mut value: i32 = tokens[3].parse().expect("nan");
        if tokens[2] == "lose" {
            value *= -1;
        }

        if !people.contains(&tokens[0]) {
            people.push(tokens[0]);
        }

        happiness.insert(pair, value);
    }

    let mut results: Vec<i32> = Vec::new();
    for perm in people.iter().permutations(people.len()) {
        let mut value = 0;
        for (person, next_person) in perm[..people.len() - 1].iter().zip(&perm[1..]) {
            value += happiness.get(&(person, next_person)).unwrap();
            value += happiness.get(&(next_person, person)).unwrap();
        }

        value += happiness
            .get(&(perm.first().unwrap(), perm.last().unwrap()))
            .unwrap();
        value += happiness
            .get(&(perm.last().unwrap(), perm.first().unwrap()))
            .unwrap();

        results.push(value);
    }

    println!("{}", results.iter().max().unwrap());
}

pub fn part_2() {
    let input = include_str!("inputs/day13.txt");
    let mut happiness: HashMap<(&str, &str), i32> = HashMap::new();
    let mut people: Vec<&str> = vec!["Axis"];

    for line in input.lines() {
        let line = &line[..line.len() - 1];
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let pair = (tokens[0], tokens[10]);
        let mut value: i32 = tokens[3].parse().expect("nan");
        if tokens[2] == "lose" {
            value *= -1;
        }

        if !people.contains(&tokens[0]) {
            people.push(tokens[0]);
        }

        happiness.insert(pair, value);
    }

    for person in &people {
        happiness.insert(("Axis", person), 0);
        happiness.insert((person, "Axis"), 0);
    }

    let mut results: Vec<i32> = Vec::new();
    for perm in people.iter().permutations(people.len()) {
        let mut value = 0;
        for (person, next_person) in perm[..people.len() - 1].iter().zip(&perm[1..]) {
            value += happiness.get(&(person, next_person)).unwrap();
            value += happiness.get(&(next_person, person)).unwrap();
        }

        value += happiness
            .get(&(perm.first().unwrap(), perm.last().unwrap()))
            .unwrap();
        value += happiness
            .get(&(perm.last().unwrap(), perm.first().unwrap()))
            .unwrap();

        results.push(value);
    }

    println!("{}", results.iter().max().unwrap());
}
