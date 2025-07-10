pub fn part_1() {
    fn look_and_say(n: u8, seq: String) -> String {
        if n == 0 {
            return seq;
        }

        let mut new = String::new();
        let mut chars = seq.chars().peekable();

        while let Some(c) = chars.next() {
            let mut count = 1;
            while chars.peek() == Some(&c) {
                chars.next();
                count += 1;
            }

            new.push_str(format!("{count}{c}").as_str());
        }

        look_and_say(n - 1, new)
    }

    let input = include_str!("inputs/day10.txt");

    println!("{}", look_and_say(40, String::from(input)).len());
}

pub fn part_2() {
    fn look_and_say(n: u8, seq: String) -> String {
        if n == 0 {
            return seq;
        }

        let mut new = String::new();
        let mut chars = seq.chars().peekable();

        while let Some(c) = chars.next() {
            let mut count = 1;
            while chars.peek() == Some(&c) {
                chars.next();
                count += 1;
            }

            new.push_str(format!("{count}{c}").as_str());
        }

        look_and_say(n - 1, new)
    }

    let input = include_str!("inputs/day10.txt");

    println!("{}", look_and_say(50, String::from(input)).len());
}
