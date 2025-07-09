pub fn part_1() {
    let input = include_str!("inputs/day5.txt");
    let mut n_nice_strings = 0;

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        let mut vowels = 0;
        let mut prev_char = chars[0];
        let mut found_double = false;

        let contains_blacklisted_pairs = line.contains("ab")
            || line.contains("cd")
            || line.contains("pq")
            || line.contains("xy");

        if contains_blacklisted_pairs {
            continue;
        }

        for (index, c) in chars.iter().enumerate() {
            if index > 0 && c == &prev_char {
                found_double = true;
            }

            if "aeiou".contains(*c) {
                vowels += 1;
            }
            prev_char = *c;
        }

        if found_double && vowels >= 3 {
            n_nice_strings += 1;
        }
    }

    println!("{n_nice_strings}");
}

pub fn part_2() {
    let input = include_str!("inputs/day5.txt");
    // let input = "qjhvhtzxzqqjkmpb";
    let mut n_nice_strings = 0;

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        let mut condition1 = false;
        let mut condition2 = false;

        for (index, c) in chars.iter().enumerate() {
            if index < 2 {
                continue;
            }

            if &chars[index - 2] == c {
                condition2 = true;
            }
        }

        let mut vec2 = chars.clone();
        vec2.remove(0);

        let pairs: Vec<(&char, char)> = chars.iter().zip(vec2).collect();

        for (index1, pair1) in pairs.iter().enumerate() {
            for (index2, pair2) in pairs.iter().enumerate() {
                if pair1 == pair2 && index1 != index2 {
                    condition1 = true;
                }
            }
        }

        // println!("{condition1} {condition2}");
        if condition1 && condition2 {
            n_nice_strings += 1;
        }
    }

    println!("{n_nice_strings}");
}
