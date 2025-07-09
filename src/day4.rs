use md5;

pub fn part_1() {
    let input = include_str!("inputs/day4.txt");
    let mut answer = 0;
    loop {
        let hash = md5::compute(format!("{input}{answer}").as_bytes());
        let hash = format!("{:x}", hash);
        if hash.starts_with("00000") {
            break;
        }
        answer += 1;
    }
    println!("{answer}");
}

pub fn part_2() {
    let input = include_str!("inputs/day4.txt");
    let mut answer = 0;
    loop {
        let hash = md5::compute(format!("{input}{answer}").as_bytes());
        let hash = format!("{:x}", hash);
        if hash.starts_with("000000") {
            break;
        }
        answer += 1;
    }
    println!("{answer}");
}
