use serde_json::Value;

pub fn part_1() {
    let input = include_str!("inputs/day12.txt");
    let json = serde_json::from_str(input).unwrap();

    fn sum_all_numbers_in_json(value: &Value) -> i64 {
        match value {
            Value::Number(n) => n.as_i64().unwrap(),
            Value::Array(arr) => arr.iter().map(sum_all_numbers_in_json).sum(),
            Value::Object(obj) => obj.values().map(sum_all_numbers_in_json).sum(),
            _ => 0,
        }
    }

    let sum = sum_all_numbers_in_json(&json);
    println!("{sum}");
}

pub fn part_2() {
    let input = include_str!("inputs/day12.txt");
    let json = serde_json::from_str(input).unwrap();

    fn sum_all_numbers_in_json(value: &Value) -> i64 {
        match value {
            Value::Number(n) => n.as_i64().unwrap(),
            Value::Array(arr) => arr.iter().map(sum_all_numbers_in_json).sum(),
            Value::Object(obj) => {
                let mut sum = 0;
                for v in obj.values() {
                    if v == "red" {
                        return 0;
                    }
                    sum += sum_all_numbers_in_json(v);
                }

                sum
            }
            _ => 0,
        }
    }

    let sum = sum_all_numbers_in_json(&json);
    println!("{sum}");
}
