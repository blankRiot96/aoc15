#![allow(unused)]

use std::cmp;
use itertools::{Itertools, repeat_n};
use log::{trace, info, debug};
use env_logger;

struct Boss {
    hp: i32,
    dmg: i32
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

fn apply_effects(active_effects: &mut Vec<i32>, active_timers: &mut Vec<i32>, player: &mut Player, boss: &mut Boss) {
    for (index, effect) in active_effects.iter().enumerate() {
        match effect {
            2 => { player.armor += 7; trace!("Shield gave +7 armor!"); },
            3 => { boss.hp -= 3; trace!("Poison caused 3 damage!"); },
            4 => { player.mana += 101; trace!("Regenerated 101 mana!"); },
            _ => panic!("Invalid effect!"),
        };
        
        trace!("Timer: {}", active_timers[index]);
       active_timers[index] -= 1;
    }

    for (index, timer) in active_timers.clone().iter().enumerate() {
        if *timer == 0 {
            active_effects.remove(index);
            active_timers.remove(index);
        }
    }
}

fn analyze_battle(moves: &Vec<i32>) -> BattleData {
    let mut data = BattleData { won: false, cost: 0 };

    let mut player = Player { hp: 50, mana: 500, armor: 0 };
    let mut boss = Boss { hp: 55, dmg: 8 };

    let mut active_effects = Vec::new();
    let mut active_timers = Vec::new();

    'battle: for mov in moves {
        trace!("---- Player Turn ----");
        match mov {
            0 => {
                trace!("Cast Magic Missle! 4 damage!");
                player.mana -= 53;
                data.cost += 53;
                boss.hp -= 4;
            },
            1 => {
                trace!("Cast Drain Effect! 2 Damage, 2 Heal!");
                player.mana -= 73;
                data.cost += 73;
                boss.hp -= 2;
                player.hp += 2;
            },
            2 => {
                if active_effects.contains(&2) {
                    continue 'battle;
                }
                
                trace!("Started Shield Effect!");

                player.mana -= 113;
                data.cost += 113;
                active_effects.push(2);
                active_timers.push(6);
            },
            3 => {
                if active_effects.contains(&3) {
                    continue 'battle;
                }

                trace!("Started Poison Effect!");

                player.mana -= 173;
                data.cost += 173;
                active_effects.push(3);
                active_timers.push(6);
            },
            4 => {
                if active_effects.contains(&4) {
                    continue 'battle;
                }
                
                trace!("Started Recharge Effect!");

                player.mana -= 229;
                data.cost += 229;
                active_effects.push(4);
                active_timers.push(5);
            },
            _ => panic!("Invalid move!"),
        };
        apply_effects(&mut active_effects, &mut active_timers, &mut player, &mut boss);

        trace!("Boss HP: {}, Player HP: {}", boss.hp, player.hp);


        trace!("---- Boss Turn ----");
        apply_effects(&mut active_effects, &mut active_timers, &mut player, &mut boss);


        let dmg = cmp::max(boss.dmg - player.armor, 1);
        trace!("Boss deals {dmg} damage!");
        player.hp -= dmg;
        
        trace!("Boss HP: {}, Player HP: {}", boss.hp, player.hp);

        // End conditions
        if boss.hp <= 0 {
            trace!(">>> Boss Dies! <<<");
            data.won = true;
            break 'battle;
        }
        else if player.hp <= 0  || player.mana <= 0 {
            trace!(">>> Player Dies! <<<");
            break 'battle;
        }
    }
    
    trace!(">>> RAN OUT OF MOVES <<<");
    return data;
}

pub fn part_1() {
    env_logger::init();
    let mut min_cost = i32::MAX;
    for moves in repeat_n((0..5).rev(), 10).multi_cartesian_product() {
    // for moves in [vec![3, 4, 4, 4, 4, 3, 0, 0, 3, 0]] {
    // for moves in [vec![3, 0]] {
        
        trace!("{moves:?}");
        let data = analyze_battle(&moves);
        if data.won && data.cost < min_cost {
            debug!("{moves:?}, {}", data.cost);
            min_cost = data.cost;
        }
    }

    info!("{min_cost}");
}
