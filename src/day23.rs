#![allow(unused)]

use env_logger;
use log::{debug, info, trace};

#[derive(Debug)]
struct Instruction {
    name: String,
    reg_index: isize,
    offset: isize,
}

fn parse_instructions(input: String) -> Vec<Instruction> {
    let mut res = Vec::new();
    for line in input.lines() {
        let (instruction, args) = line.split_once(" ").unwrap();
        let mut reg_char = 'a';
        let mut reg_index = 0isize;
        let mut offset = 0;

        match instruction {
            "jmp" => {
                offset = args.parse::<isize>().unwrap();
            }
            "jio" | "jie" => {
                let values = split_args(args.to_string());
                reg_char = values.0.chars().next().unwrap();
                offset = values.1;
            }
            _ => {
                reg_char = args.trim().chars().next().unwrap();
            }
        }

        match reg_char {
            'a' => {
                reg_index = 0;
            }
            'b' => {
                reg_index = 1;
            }
            _ => panic!("Invalid Register"),
        }

        res.push(Instruction {
            name: instruction.to_string(),
            reg_index,
            offset,
        });
    }

    return res;
}

fn split_args(args: String) -> (String, isize) {
    let (reg, offset_str) = args.split_once(",").unwrap();
    (reg.to_string(), offset_str.trim().parse::<isize>().unwrap())
}

fn hlf(reg: &mut [i32; 2], reg_index: isize) {
    reg[reg_index as usize] /= 2;
}

fn tpl(reg: &mut [i32; 2], reg_index: isize) {
    reg[reg_index as usize] *= 3;
}

fn inc(reg: &mut [i32; 2], reg_index: isize) {
    reg[reg_index as usize] += 1;
}

fn compute(reg: &mut [i32; 2], instructions: Vec<Instruction>) {
    let mut rip = 0isize;

    while (rip as usize) < instructions.len() {
        let instruction = &instructions[rip as usize];

        match instruction.name.as_str() {
            "hlf" => hlf(reg, instruction.reg_index),
            "tpl" => tpl(reg, instruction.reg_index),
            "inc" => inc(reg, instruction.reg_index),
            "jmp" => {
                rip += instruction.offset;
                continue;
            }
            "jie" => {
                if reg[instruction.reg_index as usize] % 2 == 0 {
                    rip += instruction.offset;
                    continue;
                }
            }
            "jio" => {
                if reg[instruction.reg_index as usize] == 1 {
                    rip += instruction.offset;
                    continue;
                }
            }
            _ => panic!("Invalid Instruction"),
        }

        rip += 1;
    }
}

pub fn part_1() {
    env_logger::init();

    let input = include_str!("inputs/day23.txt");
    let instructions = parse_instructions(input.to_string());
    let mut reg = [0, 0];
    compute(&mut reg, instructions);
    info!("{}", reg[1]);
}

pub fn part_2() {
    env_logger::init();

    let input = include_str!("inputs/day23.txt");
    let instructions = parse_instructions(input.to_string());
    let mut reg = [1, 0];
    compute(&mut reg, instructions);
    info!("{}", reg[1]);
}
