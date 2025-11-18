use itertools::Itertools;
use std::cmp;

pub fn part_1() {
    // I mean, I just looked at the shop and figured it out lol
    // Since the boss has 1 Armour and 8 Dmg, by purchasing
    // 1 Armour of our own, we get a leeway of 15 strikes until we are
    // killed.
    //
    // So basically we need to kill the boss in 15 strikes. We need to inflict
    // 7 damage (7 * 15 = 105), but since the boss has 1 Armour, we need 8 damage.
    //
    // `long_sword` offers 7 damage, and then we use a ring buff to get 1 damage more
    // And we need the leather (1 Armour) so that we can get a leeway of
    // 15 available strikes

    let long_sword = 40;
    let leather = 13;
    let ring_dmg_plus_one = 25;
    println!("{}", long_sword + leather + ring_dmg_plus_one);
}

#[derive(Debug)]
struct Item {
    cost: u32,
    damage: u32,
    armor: u32,
}

impl Item {
    fn new(cost: u32, damage: u32, armor: u32) -> Self {
        Self {
            cost,
            damage,
            armor,
        }
    }
}

fn would_win(items: &Vec<&Item>) -> bool {
    let boss_hp =  104;
    let mut boss_dmg = 8;
    let boss_armor = 1;
    
    let player_hp = 100;
    let mut player_dmg = items.iter().fold(0, |acc, item| { acc + item.damage }) as i32;
    let player_armor = items.iter().fold(0, |acc, item| { acc + item.armor }) as i32;

    boss_dmg = cmp::max(boss_dmg - player_armor, 1);
    player_dmg = cmp::max(player_dmg - boss_armor, 1);

    let strikes = (player_hp / boss_dmg) + 1;
    
    strikes * player_dmg >= boss_hp
}

fn total_cost(items: &Vec<&Item>) -> u32 {
    return items.iter().fold(0, | acc, item | { acc + item.cost });
}

fn swap_max(items: &Vec<&Item>, max_cost: &mut u32) {
     if !would_win(&items) {
        let total = total_cost(&items);
        if total > *max_cost {
            *max_cost = total;
            println!("{:?}", items);
        }
    }   
}

pub fn part_2() {
    let weapons = vec![
        Item::new(8, 4, 0),
        Item::new(10, 5, 0),
        Item::new(25, 6, 0),
        Item::new(40, 7, 0),
        Item::new(74, 8, 0),
    ];
    
    let armors = vec![
        Item::new(13, 0, 1),
        Item::new(31, 0, 2),
        Item::new(53, 0, 3),
        Item::new(75, 0, 4),
        Item::new(102, 0, 5),
    ];

    let rings = vec![
        Item::new(25, 1, 0),
        Item::new(50, 2, 0),
        Item::new(100, 3, 0),

        Item::new(20, 0, 1),
        Item::new(40, 0, 2),
        Item::new(80, 0, 3),
    ];
    
    let mut max_cost = 0u32;
    for weapon in &weapons {
        let items = vec![weapon];
        swap_max(&items, &mut max_cost);

        for ring in &rings {
            let items = vec![weapon, ring];
            swap_max(&items, &mut max_cost);
        }

        for ring_comb in rings.iter().combinations(2) {
            let mut items = vec![weapon];
            items.extend(ring_comb);
            swap_max(&items, &mut max_cost);
        }

        for armor in &armors {
            let items = vec![weapon, armor];
            swap_max(&items, &mut max_cost);
    
            for ring in &rings {
                let items = vec![weapon, armor, ring];
                swap_max(&items, &mut max_cost);
            }

            for ring_comb in rings.iter().combinations(2) {
                let mut items = vec![weapon, armor];
                items.extend(ring_comb);
                swap_max(&items, &mut max_cost);
            }
        }
    }

    println!("{max_cost}");
}
