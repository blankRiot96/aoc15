pub fn part_1() {
    let input = include_str!("inputs/day3.txt");
    let mut current_location = (0, 0);
    let mut saved_locations = Vec::new();
    let mut unique_houses = 0;
    for direction in input.chars() {
        match direction {
            '^' => current_location.1 += 1,
            '>' => current_location.0 += 1,
            'v' => current_location.1 -= 1,
            '<' => current_location.0 -= 1,
            _ => println!("invalid direction found"),
        }

        if saved_locations.contains(&current_location) {
            continue;
        }

        saved_locations.push(current_location);
        unique_houses += 1;
    }

    println!("{unique_houses}");
}

pub fn part_2() {
    fn location_updater(
        direction: &char,
        current_location: &mut (i32, i32),
        saved_locations: &mut Vec<(i32, i32)>,
        unique_houses: &mut i32,
    ) {
        match direction {
            '^' => current_location.1 += 1,
            '>' => current_location.0 += 1,
            'v' => current_location.1 -= 1,
            '<' => current_location.0 -= 1,
            _ => println!("invalid direction found"),
        }

        if saved_locations.contains(&current_location) {
            return;
        }

        saved_locations.push(*current_location);
        *unique_houses += 1;
    }

    let input = include_str!("inputs/day3.txt");
    let mut santa_location = (0, 0);
    let mut robo_location = (0, 0);
    let mut saved_locations: Vec<(i32, i32)> = Vec::new();
    let mut unique_houses = 0;

    for (index, direction) in input.chars().enumerate() {
        if index % 2 == 0 {
            location_updater(
                &direction,
                &mut santa_location,
                &mut saved_locations,
                &mut unique_houses,
            );
        } else {
            location_updater(
                &direction,
                &mut robo_location,
                &mut saved_locations,
                &mut unique_houses,
            );
        }
    }

    println!("{unique_houses}");
}
