use itertools::Itertools;

pub fn part_1() {
    let input = include_str!("inputs/day09.txt");
    let mut distances: Vec<(&str, i32, &str)> = Vec::new();
    let mut places: Vec<&str> = Vec::new();

    for line in input.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let start = tokens[0];
        let end = tokens[2];
        let distance = tokens[4].parse().expect("nan");

        if !places.contains(&start) {
            places.push(start);
        }
        if !places.contains(&end) {
            places.push(end);
        }

        distances.push((start, distance, end));
    }

    let mut results = Vec::new();
    for perm in places.iter().permutations(places.len()) {
        let mut acc = 0;
        let mut valid = true;

        for i in 0..perm.len() - 1 {
            let a = perm[i];
            let b = perm[i + 1];

            if let Some((_, dist, _)) = distances
                .iter()
                .find(|(x, _, y)| (&x == &a && &y == &b) || (&x == &b && &y == &a))
            {
                acc += dist;
            } else {
                valid = false;
                break;
            }
        }

        if valid {
            results.push(acc);
        }
    }

    println!("{}", results.iter().min().unwrap());
}

pub fn part_2() {
    let input = include_str!("inputs/day09.txt");
    let mut distances: Vec<(&str, i32, &str)> = Vec::new();
    let mut places: Vec<&str> = Vec::new();

    for line in input.lines() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let start = tokens[0];
        let end = tokens[2];
        let distance = tokens[4].parse().expect("nan");

        if !places.contains(&start) {
            places.push(start);
        }
        if !places.contains(&end) {
            places.push(end);
        }

        distances.push((start, distance, end));
    }

    let mut results = Vec::new();
    for perm in places.iter().permutations(places.len()) {
        let mut acc = 0;
        let mut valid = true;

        for i in 0..perm.len() - 1 {
            let a = perm[i];
            let b = perm[i + 1];

            if let Some((_, dist, _)) = distances
                .iter()
                .find(|(x, _, y)| (&x == &a && &y == &b) || (&x == &b && &y == &a))
            {
                acc += dist;
            } else {
                valid = false;
                break;
            }
        }

        if valid {
            results.push(acc);
        }
    }

    println!("{}", results.iter().max().unwrap());
}
