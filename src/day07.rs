use std::collections::HashMap;

pub fn part_1() {
    let input = include_str!("inputs/day07.txt");
    let mut instructions: HashMap<&str, &str> = HashMap::new();
    for line in input.lines() {
        let (expr, dest) = line.split_once(" -> ").unwrap();
        instructions.insert(dest, expr);
    }
    let mut variables: HashMap<&str, u16> = HashMap::new();

    fn eval<'a>(
        wire: &'a str,
        instructions: &HashMap<&'a str, &'a str>,
        variables: &mut HashMap<&'a str, u16>,
    ) -> u16 {
        if let Ok(n) = wire.parse::<u16>() {
            return n;
        }

        if let Some(var) = variables.get(wire) {
            return *var;
        }

        let expr = instructions.get(wire).unwrap();
        let tokens: Vec<&str> = expr.split_whitespace().collect();

        let value = match tokens.as_slice() {
            [x] => eval(x, instructions, variables),
            ["NOT", x] => !eval(x, instructions, variables),
            [x, "AND", y] => eval(x, instructions, variables) & eval(y, instructions, variables),
            [x, "OR", y] => eval(x, instructions, variables) | eval(y, instructions, variables),
            [x, "LSHIFT", y] => {
                eval(x, instructions, variables) << eval(y, instructions, variables)
            }
            [x, "RSHIFT", y] => {
                eval(x, instructions, variables) >> eval(y, instructions, variables)
            }
            _ => panic!("err"),
        };

        variables.insert(wire, value);
        value
    }

    let value = eval("a", &instructions, &mut variables);
    println!("{value}");
}

pub fn part_2() {
    let input = include_str!("inputs/day07.txt");
    let mut instructions: HashMap<&str, &str> = HashMap::new();
    for line in input.lines() {
        let (expr, dest) = line.split_once(" -> ").unwrap();
        instructions.insert(dest, expr);
    }
    let mut variables: HashMap<&str, u16> = HashMap::new();

    fn eval<'a>(
        wire: &'a str,
        instructions: &HashMap<&'a str, &'a str>,
        variables: &mut HashMap<&'a str, u16>,
    ) -> u16 {
        if let Ok(n) = wire.parse::<u16>() {
            return n;
        }

        if let Some(var) = variables.get(wire) {
            return *var;
        }

        let expr = instructions.get(wire).unwrap();
        let tokens: Vec<&str> = expr.split_whitespace().collect();

        let value = match tokens.as_slice() {
            [x] => eval(x, instructions, variables),
            ["NOT", x] => !eval(x, instructions, variables),
            [x, "AND", y] => eval(x, instructions, variables) & eval(y, instructions, variables),
            [x, "OR", y] => eval(x, instructions, variables) | eval(y, instructions, variables),
            [x, "LSHIFT", y] => {
                eval(x, instructions, variables) << eval(y, instructions, variables)
            }
            [x, "RSHIFT", y] => {
                eval(x, instructions, variables) >> eval(y, instructions, variables)
            }
            _ => panic!("err"),
        };

        variables.insert(wire, value);
        value
    }

    let value = eval("a", &instructions, &mut variables);
    let mut variables: HashMap<&str, u16> = HashMap::new();
    variables.insert("b", value);
    let value = eval("a", &instructions, &mut variables);
    println!("{value}");
}
