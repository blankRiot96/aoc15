#![allow(unused)]

use env_logger;
use itertools::{Itertools, repeat_n};
use log::{debug, info, trace};
use std::cmp;

struct Boss {
    hp: i32,
    dmg: i32,
}

struct Player {
    hp: i32,
    mana: i32,
    armor: i32,
}

struct BattleData {
    won: bool,
    cost: i32,
}

fn apply_effects(
    active_effects: &mut Vec<i32>,
    active_timers: &mut Vec<i32>,
    player: &mut Player,
    boss: &mut Boss,
) {
    player.armor = 0;
    for (index, effect) in active_effects.iter().enumerate() {
        match effect {
            2 => {
                player.armor = 7;
                trace!("Shield gave +7 armor!");
            }
            3 => {
                boss.hp -= 3;
                trace!("Poison caused 3 damage!");
            }
            4 => {
                player.mana += 101;
                trace!("Regenerated 101 mana!");
            }
            _ => panic!("Invalid effect!"),
        };

        trace!("Timer: {}", active_timers[index]);
        active_timers[index] -= 1;
    }

    let mut i = active_timers.len();
    while i > 0 {
        i -= 1;
        if active_timers[i] == 0 {
            active_timers.remove(i);
            active_effects.remove(i);
        }
    }
}

fn analyze_battle(moves: &Vec<i32>, spill: bool) -> BattleData {
    let mut data = BattleData {
        won: false,
        cost: 0,
    };

    let mut player = Player {
        hp: 50,
        mana: 500,
        armor: 0,
    };
    let mut boss = Boss { hp: 55, dmg: 8 };

    let mut active_effects = Vec::new();
    let mut active_timers = Vec::new();

    'battle: for mov in moves {
        if let Some(idx) = active_effects.iter().position(|&e| e == *mov) {
            if active_timers[idx] > 0 {
                continue 'battle;
            }
        }

        trace!("---- Player Turn ----");
        if spill {
            trace!("Spilled some blood");
            player.hp -= 1;

            if player.hp <= 0 {
                trace!(">>> Blood spillage caused player death !! <<<");
                break 'battle;
            }
        }

        apply_effects(
            &mut active_effects,
            &mut active_timers,
            &mut player,
            &mut boss,
        );

        match mov {
            0 => {
                trace!("Cast Magic Missle! 4 damage!");
                player.mana -= 53;
                data.cost += 53;
                boss.hp -= 4;
            }
            1 => {
                trace!("Cast Drain Effect! 2 Damage, 2 Heal!");
                player.mana -= 73;
                data.cost += 73;
                boss.hp -= 2;
                player.hp += 2;
            }
            2 => {
                trace!("Started Shield Effect!");
                player.mana -= 113;
                data.cost += 113;
                active_effects.push(2);
                active_timers.push(6);
            }
            3 => {
                trace!("Started Poison Effect!");
                player.mana -= 173;
                data.cost += 173;
                active_effects.push(3);
                active_timers.push(6);
            }
            4 => {
                trace!("Started Recharge Effect!");
                player.mana -= 229;
                data.cost += 229;
                active_effects.push(4);
                active_timers.push(5);
            }
            _ => panic!("Invalid move!"),
        };

        trace!("Boss HP: {}, Player HP: {}", boss.hp, player.hp);

        trace!("---- Boss Turn ----");
        apply_effects(
            &mut active_effects,
            &mut active_timers,
            &mut player,
            &mut boss,
        );

        let dmg = cmp::max(boss.dmg - player.armor, 1);
        trace!("Boss deals {dmg} damage!");
        player.hp -= dmg;

        trace!("Boss HP: {}, Player HP: {}", boss.hp, player.hp);

        // End conditions
        if boss.hp <= 0 {
            trace!(">>> Boss Dies! <<<");
            data.won = true;
            break 'battle;
        } else if player.hp <= 0 || player.mana <= 0 {
            trace!(">>> Player Dies! <<<");
            break 'battle;
        }
    }

    trace!(">>> END OF FUNCTION <<<");
    return data;
}

pub fn part_1() {
    env_logger::init();
    let mut min_cost = i32::MAX;
    for moves in repeat_n((0..5).rev(), 10).multi_cartesian_product() {
        // for moves in [vec![3, 4, 4, 4, 4, 3, 0, 0, 3, 0]] {
        // for moves in [vec![3, 0]] {
        trace!("{moves:?}");
        let data = analyze_battle(&moves, false);
        if data.won && data.cost < min_cost {
            debug!("{moves:?}, {}", data.cost);
            min_cost = data.cost;
        }
    }

    info!("{min_cost}");
}

pub fn part_2() {
    env_logger::init();
    debug!("p2?");
    let mut min_cost = i32::MAX;
    // for moves in repeat_n((0..5).rev(), 10).multi_cartesian_product() {
    for moves in [vec![3, 4, 2, 0, 3, 4, 2, 0, 3, 0]] {
        // for moves in [vec![3, 0]] {
        trace!("{moves:?}");
        let data = analyze_battle(&moves, true);
        if data.won && data.cost < min_cost {
            debug!("{moves:?}, {}", data.cost);
            min_cost = data.cost;
        }
    }

    info!("{min_cost}");
}
