pub fn part_1() {
    fn check_password(password: &str) -> bool {
        let bytes = password.as_bytes();

        let has_straight = bytes
            .windows(3)
            .any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2]);

        if !has_straight {
            return false;
        }

        if password.contains(['i', 'o', 'l']) {
            return false;
        }

        let mut pairs = 0;
        let mut i = 0;
        while i + 1 < bytes.len() {
            if bytes[i] == bytes[i + 1] {
                pairs += 1;
                i += 2;
            } else {
                i += 1;
            }
        }

        pairs >= 2
    }

    fn increment_password(password: &str) -> String {
        let mut chars: Vec<u8> = password.bytes().collect();
        for i in (0..chars.len()).rev() {
            if chars[i] == b'z' {
                chars[i] = b'a';
            } else {
                chars[i] += 1;
                break;
            }
        }
        String::from_utf8(chars).unwrap()
    }

    let mut password = String::from(include_str!("inputs/day11.txt"));
    while !check_password(&password) {
        password = increment_password(&password);
    }

    println!("{password}");
}

pub fn part_2() {
    fn check_password(password: &str) -> bool {
        let bytes = password.as_bytes();

        let has_straight = bytes
            .windows(3)
            .any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2]);

        if !has_straight {
            return false;
        }

        if password.contains(['i', 'o', 'l']) {
            return false;
        }

        let mut pairs = 0;
        let mut i = 0;
        while i + 1 < bytes.len() {
            if bytes[i] == bytes[i + 1] {
                pairs += 1;
                i += 2;
            } else {
                i += 1;
            }
        }

        pairs >= 2
    }

    fn increment_password(password: &str) -> String {
        let mut chars: Vec<u8> = password.bytes().collect();
        for i in (0..chars.len()).rev() {
            if chars[i] == b'z' {
                chars[i] = b'a';
            } else {
                chars[i] += 1;
                break;
            }
        }
        String::from_utf8(chars).unwrap()
    }

    let mut password = String::from(include_str!("inputs/day11.txt"));
    while !check_password(&password) {
        password = increment_password(&password);
    }

    password = increment_password(&password);
    while !check_password(&password) {
        password = increment_password(&password);
    }

    println!("{password}");
}
