#![allow(unused)]

pub fn part_1() {
    let input = include_str!("inputs/day20.txt")
        .trim()
        .parse::<u64>()
        .unwrap();
    let n = (input / 10) as usize;

    // let input = 130;
    // let n = 130;
    let mut houses: Vec<u64> = vec![0; n];

    for i in 1..=n {
        let mut house_index = i - 1;
        while house_index < n {
            houses[house_index] += (i as u64) * 10;
            house_index += i;
        }
    }

    for (index, house) in houses.iter().enumerate() {
        if house >= &input {
            let house_number = index + 1;
            println!("{house_number}");
            break;
        }
    }
}

pub fn part_2() {
    let input = include_str!("inputs/day20.txt")
        .trim()
        .parse::<u64>()
        .unwrap();
    let n = input as usize;

    // let input = 130;
    // let n = 130;
    let mut houses: Vec<u64> = vec![0; n];

    for i in 1..=n {
        let mut house_index = i - 1;
        let mut count = 0;
        while count < 50 && house_index < n {
            houses[house_index] += (i as u64) * 11;
            house_index += i;
            count += 1;
        }
    }

    for (index, house) in houses.iter().enumerate() {
        if house >= &input {
            let house_number = index + 1;
            println!("{house_number}");
            break;
        }
    }
}
