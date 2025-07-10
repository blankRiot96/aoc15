use collar::CollectArray;

pub fn part_1() {
    let mut result = 0;
    let data = include_str!("inputs/day02.txt");
    let lines = data.lines();

    for input in lines {
        let mut sides: [u32; 3] = input
            .split('x')
            .map(|side| side.parse().expect("Invalid number"))
            .collect_array();

        sides.sort();
        let [l, w, h] = sides;
        let slack = l * w;
        let surface_area = 2 * (l * w + w * h + h * l);

        result += surface_area + slack;
    }

    println!("{result}");
}

pub fn part_2() {
    let mut result = 0;
    let data = include_str!("inputs/day02.txt");
    let lines = data.lines();

    for input in lines {
        let mut sides: [u32; 3] = input
            .split('x')
            .map(|side| side.parse().expect("Invalid number"))
            .collect_array();

        sides.sort();
        let [l, w, h] = sides;

        let bow_length = l * w * h;
        let smallest_perimeter = 2 * (l + w);

        result += bow_length + smallest_perimeter;
    }

    println!("{result}");
}
