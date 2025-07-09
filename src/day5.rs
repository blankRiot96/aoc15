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
