#![allow(unused)]

use std::fs;

fn read_input(file_path: String) -> Vec<Vec<u32>> {
    let mut grid = Vec::new();
    for line in fs::read_to_string(file_path)
        .expect("Error! Error! File reading failed?!")
        .lines()
    {
        let row: Vec<u32> = line
            .chars()
            .map(|c| match c {
                '#' => 1,
                '.' => 0,
                _ => panic!("Invalid character!"),
            })
            .collect();
        grid.push(row);
    }
    return grid;
}

fn get_neighbours(row: i32, col: i32, grid: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut neighbours = Vec::new();

    for r in row - 1..=row + 1 {
        for c in col - 1..=col + 1 {
            if r == row && c == col {
                continue;
            }

            if r < 0 || r >= 100 || c < 0 || c >= 100 {
                continue;
            }

            neighbours.push(grid[r as usize][c as usize]);
        }
    }

    return neighbours;
}

fn get_new_state(row: i32, col: i32, grid: &Vec<Vec<u32>>) -> u32 {
    let light = grid[row as usize][col as usize];
    let neighbours = get_neighbours(row, col, grid);
    let horny_neighbours: i16 = neighbours.iter().map(|x| *x as i16).sum();

    // println!("{:?}", neighbours);
    if light == 1 && !([2, 3].contains(&horny_neighbours)) {
        // println!("Always happen? {} {} horny: {}", row, col, horny_neighbours);
        return 0;
    } else if light == 0 && horny_neighbours == 3 {
        return 1;
    }

    return light;
}

fn step(grid: &mut Vec<Vec<u32>>) {
    let original_grid = grid.clone();
    for row in 0..100 {
        for col in 0..100 {
            grid[row][col] = get_new_state(row as i32, col as i32, &original_grid);
        }
    }
}

pub fn part_1() {
    let mut grid = read_input("src/inputs/day18.txt".to_string());

    for _ in 0..100 {
        step(&mut grid);
    }

    let answer: u32 = grid
        .iter()
        .fold(0, |sum, row| sum + row.iter().sum::<u32>());
    println!("{}", answer);
}

fn get_new_state_p2(row: i32, col: i32, grid: &Vec<Vec<u32>>) -> u32 {
    for row_lim in [0, 99] {
        for col_lim in [0, 99] {
            if row == row_lim && col == col_lim {
                return 1;
            }
        }
    }

    let light = grid[row as usize][col as usize];
    let neighbours = get_neighbours(row, col, grid);
    let horny_neighbours: i16 = neighbours.iter().map(|x| *x as i16).sum();

    // println!("{:?}", neighbours);
    if light == 1 && !([2, 3].contains(&horny_neighbours)) {
        // println!("Always happen? {} {} horny: {}", row, col, horny_neighbours);
        return 0;
    } else if light == 0 && horny_neighbours == 3 {
        return 1;
    }

    return light;
}

fn step_p2(grid: &mut Vec<Vec<u32>>) {
    let original_grid = grid.clone();
    for row in 0..100 {
        for col in 0..100 {
            grid[row][col] = get_new_state_p2(row as i32, col as i32, &original_grid);
        }
    }
}

pub fn part_2() {
    let mut grid = read_input("src/inputs/day18.txt".to_string());

    for _ in 0..100 {
        step_p2(&mut grid);
    }

    let answer: u32 = grid
        .iter()
        .fold(0, |sum, row| sum + row.iter().sum::<u32>());
    println!("{}", answer);
}
