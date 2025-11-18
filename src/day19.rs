#![allow(unused)]

use std::collections::{HashMap, HashSet};

fn replace_one_each(s: &str, pattern: &str, replacement: &str) -> Vec<String> {
    let mut results = Vec::new();
    let mut start = 0;

    while let Some(pos) = s[start..].find(pattern) {
        let pos = start + pos;
        let mut new_string = String::new();
        new_string.push_str(&s[..pos]);
        new_string.push_str(replacement);
        new_string.push_str(&s[pos + pattern.len()..]);
        results.push(new_string);

        start = pos + 1;
    }

    results
}

pub fn part_1() {
    let input = include_str!("inputs/day19.txt");
    let (replacements_content, calibration_molecule) = input.split_once("\n\n").unwrap();

    let mut replacements: Vec<(String, String)> = Vec::new();
    for line in replacements_content.lines() {
        let (subject, replacement) = line.split_once("=>").unwrap();
        replacements.push((subject.trim().to_string(), replacement.trim().to_string()));
    }

    let mut unique: HashSet<String> = HashSet::new();
    for (subject, replacement) in replacements.iter() {
        let pos = replace_one_each(calibration_molecule, subject, replacement);
        for p in pos {
            unique.insert(p);
        }
    }

    println!("{}", unique.iter().len());
}

pub fn part_2() {
    let input = include_str!("inputs/day19.txt");
    let (replacements_content, calibration_molecule) = input.split_once("\n\n").unwrap();

    let mut replacements: Vec<(String, String)> = Vec::new();
    for line in replacements_content.lines() {
        let (replacement, subject) = line.split_once("=>").unwrap();
        replacements.push((subject.trim().to_string(), replacement.trim().to_string()));
    }
    
    let mut molecule = calibration_molecule.to_string();
    let mut answer = 0;

    loop {
        let mut no_replacements = true;
        for (subject, replacement) in &replacements {
            let new = molecule.replacen(subject, replacement, 1);

            if new != molecule {
                answer += 1;
                no_replacements = false;
                molecule = new;
            }
        }

        if no_replacements {
            break;
        }
    }

    println!("{answer}");
}
