#![allow(unused)]

use itertools::{Itertools, repeat_n};
use std::cmp;

struct Player {
    hp: u16,
    mana: u16,
    armor: u16,
}

struct Boss {
    hp: u16,
    damage: u16,
}

trait Spell {
    fn get_cost(&self) -> u16;
    fn cast(&self, player: &mut Player, boss: &mut Boss) -> bool;
}

trait Effect {
    fn get_initiated(&self) -> bool;
    fn set_initiated(&mut self, val: bool);
    fn get_cost(&self) -> u16;
    fn get_timer(&self) -> u16;
    fn initiate(&mut self, player: &mut Player);
    fn cast(&mut self, player: &mut Player, boss: &mut Boss) -> bool;
}

struct MagicMissle {
    cost: u16,
    damage: u16,
}

impl Spell for MagicMissle {
    fn get_cost(&self) -> u16 {
        return self.cost;
    }

    fn cast(&self, player: &mut Player, boss: &mut Boss) -> bool {
        if (boss.hp as i16) - (self.damage as i16) < 0 {
            boss.hp = 0;
            return false;
        }

        player.mana -= self.cost;
        boss.hp -= self.damage;

        return true;
    }
}

struct Drain {
    cost: u16,
    damage: u16,
    heal: u16,
}

impl Spell for Drain {
    fn get_cost(&self) -> u16 {
        return self.cost;
    }

    fn cast(&self, player: &mut Player, boss: &mut Boss) -> bool {
        if (boss.hp as i16) - (self.damage as i16) < 0 {
            boss.hp = 0;
            return false;
        }

        player.mana -= self.cost;
        boss.hp -= self.damage;

        player.hp += self.heal;
        return true;
    }
}

struct Shield {
    cost: u16,
    armor_increase: u16,
    timer: u16,
    initiated: bool,
}

impl Effect for Shield {
    fn get_initiated(&self) -> bool {
        return self.initiated;
    }

    fn set_initiated(&mut self, val: bool) {
        self.initiated = val;
    }

    fn get_timer(&self) -> u16 {
        return self.timer;
    }

    fn get_cost(&self) -> u16 {
        return self.cost;
    }

    fn initiate(&mut self, player: &mut Player) {
        player.mana -= self.cost;
        self.initiated = true;
    }

    fn cast(&mut self, player: &mut Player, boss: &mut Boss) -> bool {
        player.armor += self.armor_increase;
        self.timer -= 1;

        return true;
    }
}

struct Poison {
    cost: u16,
    damage: u16,
    timer: u16,
    initiated: bool,
}

impl Effect for Poison {
    fn get_initiated(&self) -> bool {
        return self.initiated;
    }

    fn set_initiated(&mut self, val: bool) {
        self.initiated = val;
    }
    fn get_timer(&self) -> u16 {
        return self.timer;
    }

    fn get_cost(&self) -> u16 {
        return self.cost;
    }

    fn initiate(&mut self, player: &mut Player) {
        player.mana -= self.cost;
        self.initiated = true;
    }

    fn cast(&mut self, player: &mut Player, boss: &mut Boss) -> bool {
        if (boss.hp as i16) - (self.damage as i16) < 0 {
            boss.hp = 0;
            return false;
        }

        boss.hp -= self.damage;
        self.timer -= 1;

        return true;
    }
}

struct Recharge {
    cost: u16,
    mana_increase: u16,
    timer: u16,
    initiated: bool,
}

impl Effect for Recharge {
    fn get_initiated(&self) -> bool {
        return self.initiated;
    }
    fn set_initiated(&mut self, val: bool) {
        self.initiated = val;
    }
    fn get_timer(&self) -> u16 {
        return self.timer;
    }

    fn get_cost(&self) -> u16 {
        return self.cost;
    }

    fn initiate(&mut self, player: &mut Player) {
        player.mana -= self.cost;
        self.initiated = true;
    }

    fn cast(&mut self, player: &mut Player, boss: &mut Boss) -> bool {
        player.mana += self.mana_increase;
        self.timer -= 1;

        return true;
    }
}

pub fn part_1<'a>() {
    let mut min_cost = 9999;
    'main: for perm in repeat_n((0..5).rev(), 15).multi_cartesian_product() {
        // let perm = [4, 2, 3, , 0, 0, 0, 0, 0, 0, 0, 0];
        let mut player = Player {
            hp: 50,
            mana: 1000,
            armor: 0,
        };
        let mut boss = Boss { hp: 55, damage: 8 };

        let mut spells: Vec<Box<dyn Spell>> = vec![
            Box::new(MagicMissle {
                cost: 53,
                damage: 4,
            }),
            Box::new(Drain {
                cost: 73,
                damage: 2,
                heal: 2,
            }),
        ];
        let mut effects: Vec<Box<dyn Effect>> = vec![
            Box::new(Shield {
                cost: 113,
                armor_increase: 7,
                timer: 6,
                initiated: false,
            }),
            Box::new(Poison {
                cost: 173,
                damage: 3,
                timer: 6,
                initiated: false,
            }),
            Box::new(Recharge {
                cost: 229,
                mana_increase: 101,
                timer: 5,
                initiated: false,
            }),
        ];

        let mut cost_acc = 0;
        println!("{perm:?}");
        'battle: for i in &perm {
            for effect in &mut effects {
                if effect.get_initiated() {
                    if effect.get_timer() == 0 {
                        effect.set_initiated(false);
                        continue;
                    }
                    if !effect.cast(&mut player, &mut boss) {
                        println!("Victory!");
                        if cost_acc < min_cost {
                            min_cost = cost_acc;
                            continue 'battle;
                        }
                        break 'main;
                    }
                }
            }
            if *i > 1 {
                if (player.mana as i16) - (effects[*i - 2].get_cost() as i16) < 0 {
                    continue 'battle;
                }

                effects[*i - 2].initiate(&mut player);
                cost_acc += effects[*i - 2].get_cost();
            } else {
                if (player.mana as i16) - (spells[*i].get_cost() as i16) < 0 {
                    println!("Cant afford item!");
                    continue 'battle;
                }

                cost_acc += spells[*i].get_cost();
                if !spells[*i].cast(&mut player, &mut boss) {
                    println!("Victory!");
                    if cost_acc < min_cost {
                        min_cost = cost_acc;
                        break 'battle;
                    }
                    break 'main;
                }
            }

            let diff = (boss.damage as i16) - (player.armor as i16);
            let dmg = cmp::max(diff, 1);
            if (player.hp as i16) - dmg < 0 {
                println!("Player dies!");
                player.hp = 0;
                break;
            }
            player.hp -= dmg as u16;
        }
        println!(
            "boss.hp={}, cost={}, player.hp={}",
            boss.hp, cost_acc, player.hp
        );
    }

    println!("{min_cost}");
}
