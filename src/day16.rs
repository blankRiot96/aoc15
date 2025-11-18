use std::collections::HashMap;
use std::fs;

fn read_input(file_path: String) -> Vec<HashMap<String, u32>> {
    let mut sues = Vec::new();

    for line in fs::read_to_string(file_path)
        .expect("Unable to read file")
        .lines()
    {
        let mut sue = HashMap::new();

        let (_, content) = line.split_once(":").unwrap();
        let pairs = content.split(",");
        for pair in pairs {
            let (category, value) = pair.split_once(":").unwrap();
            sue.insert(
                category.trim().to_string(),
                value.trim().parse::<u32>().unwrap(),
            );
        }

        sues.push(sue);
    }

    return sues;
}

pub fn part_1() {
    let sues = read_input("src/inputs/day16.txt".to_string());
    let target = HashMap::from([
        ("children".to_string(), 3),
        ("cats".to_string(), 7),
        ("samoyeds".to_string(), 2),
        ("pomeranians".to_string(), 3),
        ("akitas".to_string(), 0),
        ("vizslas".to_string(), 0),
        ("goldfish".to_string(), 5),
        ("trees".to_string(), 3),
        ("cars".to_string(), 2),
        ("perfumes".to_string(), 1),
    ]);

    let mut sue_number = 0;
    for (sue_index, sue) in sues.iter().enumerate() {
        let mut is_match = true;
        for (category, value) in target.iter() {
            if !sue.contains_key(category) {
                continue;
            }

            if sue.get(category) != Some(value) {
                is_match = false;
            }
        }

        if is_match {
            sue_number = sue_index + 1;
            break;
        }
    }

    println!("{:?}", sue_number);
}

pub fn part_2() {
    let sues = read_input("src/inputs/day16.txt".to_string());
    let target = HashMap::from([
        ("children".to_string(), 3),
        ("cats".to_string(), 7),
        ("samoyeds".to_string(), 2),
        ("pomeranians".to_string(), 3),
        ("akitas".to_string(), 0),
        ("vizslas".to_string(), 0),
        ("goldfish".to_string(), 5),
        ("trees".to_string(), 3),
        ("cars".to_string(), 2),
        ("perfumes".to_string(), 1),
    ]);

    let mut sue_number = 0;
    for (sue_index, sue) in sues.iter().enumerate() {
        let mut is_match = true;
        for (category, value) in target.iter() {
            if !sue.contains_key(category) {
                continue;
            }

            if category == "cats" || category == "trees" {
                if sue.get(category).unwrap() <= value {
                    is_match = false;
                }
            } else if category == "pomeranians" || category == "goldfish" {
                if sue.get(category).unwrap() >= value {
                    is_match = false;
                }
            } else {
                if sue.get(category) != Some(value) {
                    is_match = false;
                }
            }
        }

        if is_match {
            sue_number = sue_index + 1;
            break;
        }
    }

    println!("{:?}", sue_number);
}
