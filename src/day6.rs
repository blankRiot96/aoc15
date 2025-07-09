pub fn part_1() {
    let input = include_str!("inputs/day6.txt");

    let mut grid =
        Box::<[Box<[i32; 1000]>; 1000]>::new(std::array::from_fn(|_| Box::new([0; 1000])));

    for line in input.lines() {
        let tokens: Vec<&str> = line.split(" ").collect();
        let start_index = if tokens[0] == "turn" { 2 } else { 1 };

        let start: Vec<i32> = tokens[start_index]
            .split(",")
            .map(|x| x.parse().expect("nan"))
            .collect();
        let end: Vec<i32> = tokens[start_index + 2]
            .split(",")
            .map(|x| x.parse().expect("nan"))
            .collect();

        for col in start[0]..=end[0] {
            for row in start[1]..=end[1] {
                let row = row as usize;
                let col = col as usize;
                if tokens[0] == "turn" {
                    if tokens[1] == "off" {
                        grid[row][col] = 0;
                    } else {
                        grid[row][col] = 1;
                    }
                } else {
                    grid[row][col] = 1 - grid[row][col];
                }
            }
        }
    }

    let mut n_lights = 0;

    for row in 0..=999 {
        for col in 0..=999 {
            if grid[row][col] == 1 {
                n_lights += 1;
            }
        }
    }

    println!("{n_lights}");
}

pub fn part_2() {
    let input = include_str!("inputs/day6.txt");

    let mut grid =
        Box::<[Box<[i32; 1000]>; 1000]>::new(std::array::from_fn(|_| Box::new([0; 1000])));

    for line in input.lines() {
        let tokens: Vec<&str> = line.split(" ").collect();
        let start_index = if tokens[0] == "turn" { 2 } else { 1 };

        let start: Vec<i32> = tokens[start_index]
            .split(",")
            .map(|x| x.parse().expect("nan"))
            .collect();
        let end: Vec<i32> = tokens[start_index + 2]
            .split(",")
            .map(|x| x.parse().expect("nan"))
            .collect();

        for col in start[0]..=end[0] {
            for row in start[1]..=end[1] {
                let row = row as usize;
                let col = col as usize;
                if tokens[0] == "turn" {
                    if tokens[1] == "off" {
                        if grid[row][col] >= 1 {
                            grid[row][col] -= 1;
                        }
                    } else {
                        grid[row][col] += 1;
                    }
                } else {
                    grid[row][col] += 2;
                }
            }
        }
    }

    let mut total_brightness = 0;

    for row in 0..=999 {
        for col in 0..=999 {
            total_brightness += grid[row][col];
        }
    }

    println!("{total_brightness}");
}
