use collar;

pub fn part_1() {
    let mut result = 0;
    let data = include_str!("inputs/day2.txt");
    let lines = data.lines();

    for input in lines {
        let mut sides: Vec<u32> = input
            .split('x')
            .map(|side| side.parse().expect("Invalid number"))
            .collect();

        sides.sort();
        let [l, w, h] = collar::CollectArray::collect_array(&mut sides.iter());
        let slack = l * w;
        let surface_area = 2 * (l * w + w * h + h * l);

        result += surface_area + slack;
    }

    println!("{result}");
}

pub fn part_2() {}
