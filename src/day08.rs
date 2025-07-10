pub fn part_1() {
    fn small_parser<'t>(line: &'t str) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        let mut chars = line[1..line.len() - 1].chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '\\' => match chars.next() {
                    Some('\\') => result.push('\\' as u8),
                    Some('"') => result.push('"' as u8),
                    Some('x') => {
                        let h1 = chars.next().unwrap();
                        let h2 = chars.next().unwrap();
                        let byte = u8::from_str_radix(&format!("{h1}{h2}"), 16).unwrap();
                        result.push(byte);
                    }
                    _ => panic!("Unrecognized escape sequence :3"),
                },
                _ => result.push(c as u8),
            }
        }

        result
    }

    let input = include_str!("inputs/day08.txt");
    let mut code_chars = 0;
    let mut memory_chars = 0;
    for line in input.lines() {
        code_chars += line.len();
        memory_chars += small_parser(&line).len();
    }

    println!("{}", code_chars - memory_chars);
}

pub fn part_2() {
    fn get_n_increase<'t>(line: &'t str) -> i32 {
        let mut increase = 2;
        let mut chars = line.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '\\' => increase += 1,
                '"' => increase += 1,
                _ => continue,
            }
        }

        increase
    }

    let input = include_str!("inputs/day08.txt");
    let mut diff = 0;
    for line in input.lines() {
        diff += get_n_increase(&line);
    }

    println!("{}", diff);
}
