pub fn part_1() {
    let input = include_str!("inputs/day14.txt");
    let max = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .into_iter()
        .map(|tokens| {
            (
                tokens[3].parse().expect("nan"),
                tokens[6].parse().expect("nan"),
                tokens[13].parse().expect("nan"),
            )
        })
        .map(|(speed, fly_time, rest_time): (i32, i32, i32)| {
            speed
                * ((2503 / (fly_time + rest_time)) * fly_time
                    + (2503 % (fly_time + rest_time)).min(fly_time))
        })
        .max()
        .unwrap();

    println!("{max}");
}

pub fn part_2() {
    enum State {
        Resting(i32),
        Flying(i32),
    }

    struct Raindeer {
        pub speed: i32,
        pub fly_time: i32,
        pub rest_time: i32,
        pub points: i32,
        pub distance_covered: i32,
        pub state: State,
    }

    let input = include_str!("inputs/day14.txt");
    let mut raindeers: Vec<Raindeer> = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .into_iter()
        .map(|tokens| Raindeer {
            speed: tokens[3].parse().expect("nan"),
            fly_time: tokens[6].parse().expect("nan"),
            rest_time: tokens[13].parse().expect("nan"),
            points: 0,
            distance_covered: 0,
            state: State::Flying(0),
        })
        .collect();

    for _ in 0..=2503 {
        let mut greatest_distance = 0;
        for raindeer in &mut raindeers {
            match raindeer.state {
                State::Flying(x) => {
                    if x + 1 >= raindeer.fly_time {
                        raindeer.distance_covered += raindeer.speed;
                        raindeer.state = State::Resting(0);
                    } else {
                        raindeer.distance_covered += raindeer.speed;
                        raindeer.state = State::Flying(x + 1);
                    }
                }
                State::Resting(x) => {
                    if x + 1 >= raindeer.rest_time {
                        raindeer.state = State::Flying(0);
                    } else {
                        raindeer.state = State::Resting(x + 1);
                    }
                }
            }

            if raindeer.distance_covered > greatest_distance {
                greatest_distance = raindeer.distance_covered;
            }
        }

        for raindeer in &mut raindeers {
            if raindeer.distance_covered == greatest_distance {
                raindeer.points += 1;
            }
        }
    }

    println!(
        "{}",
        raindeers
            .iter()
            .max_by_key(|raindeer| raindeer.points)
            .unwrap()
            .points
    );
}
