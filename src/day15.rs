// ONLY WORKS FOR MY INPUT :(

use std::collections::HashMap;

pub fn part_1() {
    let input = include_str!("inputs/day15.txt");
    let mut ingredients: HashMap<&str, HashMap<&str, i64>> = HashMap::new();

    for line in input.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        let name = &tokens[0][0..tokens[0].len() - 1];
        let capacity = tokens[2][0..tokens[2].len() - 1].parse().expect("nan");
        let durability: i64 = tokens[4][0..tokens[4].len() - 1].parse().expect("nan");
        let flavor: i64 = tokens[6][0..tokens[6].len() - 1].parse().expect("nan");
        let texture: i64 = tokens[8][0..tokens[8].len() - 1].parse().expect("nan");
        let calories: i64 = tokens[10].parse().expect("nan");

        let mut properties: HashMap<&str, i64> = HashMap::new();
        properties.insert("capacity", capacity);
        properties.insert("durability", durability);
        properties.insert("flavor", flavor);
        properties.insert("texture", texture);
        properties.insert("calories", calories);

        ingredients.insert(name, properties);
    }

    let mut best = 0;
    for sprinkles in 0..=100 {
        for peanut_butter in 0..=(100 - sprinkles) {
            for frosting in 0..=(100 - peanut_butter - sprinkles) {
                let sugar = 100 - frosting - peanut_butter - sprinkles;
                let mut capacity: i64 = 0;
                let mut durability: i64 = 0;
                let mut flavor: i64 = 0;
                let mut texture: i64 = 0;

                let mut quantities: HashMap<&str, i64> = HashMap::new();
                quantities.insert("Sprinkles", sprinkles);
                quantities.insert("PeanutButter", peanut_butter);
                quantities.insert("Frosting", frosting);
                quantities.insert("Sugar", sugar);

                for (ing, properties) in ingredients.iter() {
                    capacity += quantities[ing] * properties["capacity"];
                    durability += quantities[ing] * properties["durability"];
                    flavor += quantities[ing] * properties["flavor"];
                    texture += quantities[ing] * properties["texture"];
                }

                let attempt: i64 =
                    capacity.max(0) * durability.max(0) * flavor.max(0) * texture.max(0);

                if attempt > best {
                    best = attempt;
                }
            }
        }
    }

    println!("ONLY WORKS FOR MY INPUT :(");
    println!("{best}");
}

pub fn part_2() {
    let input = include_str!("inputs/day15.txt");
    let mut ingredients: HashMap<&str, HashMap<&str, i64>> = HashMap::new();

    for line in input.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        let name = &tokens[0][0..tokens[0].len() - 1];
        let capacity = tokens[2][0..tokens[2].len() - 1].parse().expect("nan");
        let durability: i64 = tokens[4][0..tokens[4].len() - 1].parse().expect("nan");
        let flavor: i64 = tokens[6][0..tokens[6].len() - 1].parse().expect("nan");
        let texture: i64 = tokens[8][0..tokens[8].len() - 1].parse().expect("nan");
        let calories: i64 = tokens[10].parse().expect("nan");

        let mut properties: HashMap<&str, i64> = HashMap::new();
        properties.insert("capacity", capacity);
        properties.insert("durability", durability);
        properties.insert("flavor", flavor);
        properties.insert("texture", texture);
        properties.insert("calories", calories);

        ingredients.insert(name, properties);
    }

    let mut best = 0;
    for sprinkles in 0..=100 {
        for peanut_butter in 0..=(100 - sprinkles) {
            for frosting in 0..=(100 - peanut_butter - sprinkles) {
                let sugar = 100 - frosting - peanut_butter - sprinkles;
                let mut capacity: i64 = 0;
                let mut durability: i64 = 0;
                let mut flavor: i64 = 0;
                let mut texture: i64 = 0;
                let mut calories: i64 = 0;

                let mut quantities: HashMap<&str, i64> = HashMap::new();
                quantities.insert("Sprinkles", sprinkles);
                quantities.insert("PeanutButter", peanut_butter);
                quantities.insert("Frosting", frosting);
                quantities.insert("Sugar", sugar);

                for (ing, properties) in ingredients.iter() {
                    capacity += quantities[ing] * properties["capacity"];
                    durability += quantities[ing] * properties["durability"];
                    flavor += quantities[ing] * properties["flavor"];
                    texture += quantities[ing] * properties["texture"];
                    calories += quantities[ing] * properties["calories"];
                }
                
                if calories == 500 {
                    let attempt: i64 =
                        capacity.max(0) * durability.max(0) * flavor.max(0) * texture.max(0);
    
                    if attempt > best {
                        best = attempt;
                    }
                }
            }
        }
    }

    println!("ONLY WORKS FOR MY INPUT :(");
    println!("{best}");
}
