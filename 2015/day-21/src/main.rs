use day_21::{full_fight_won_by_player, get_weapons_armors_rings, Character, Item, ItemEnum};
use std::collections::HashMap;
use std::io::{self};

fn main() -> io::Result<()> {
    let files_results = vec![("input.txt", 121, 201)];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let mut values_map: HashMap<&str, i64> = HashMap::new();
        for line in file_content.iter() {
            let splitted: Vec<&str> = line.split(": ").collect();
            values_map.insert(splitted[0], splitted[1].parse::<i64>().unwrap());
        }
        let boss = Character::from_map(&values_map);
        let player = Character::new(100, 0, 0);

        let weapons_content: Vec<String> = std::fs::read_to_string("items.txt")?
            .lines()
            .map(|x| x.to_string())
            .collect();

        let (weapons, armors, rings) = get_weapons_armors_rings(&weapons_content);

        let mut current_min_cost = i64::MAX;
        let mut current_max_cost = i64::MIN;

        // Must wield a weapon
        for weapon in weapons.iter() {
            let mut player_w = player.clone();
            player_w.add_weapon(weapon);
            let cost = player_w.get_total_cost();
            if full_fight_won_by_player(player_w.clone(), boss.clone()) {
                if cost < current_min_cost {
                    current_min_cost = cost;
                }
            } else {
                if cost > current_max_cost {
                    current_max_cost = cost;
                }
            }
            for armor in armors.iter() {
                let mut player_wa = player_w.clone();
                player_wa.add_armor(armor);
                let cost = player_wa.get_total_cost();
                if full_fight_won_by_player(player_wa.clone(), boss.clone()) {
                    if cost < current_min_cost {
                        current_min_cost = cost;
                    }
                } else {
                    if cost > current_max_cost {
                        current_max_cost = cost;
                    }
                }
                for ring1 in rings.iter() {
                    let mut player_war = player_wa.clone();
                    player_war.add_rings(&ItemEnum::Ring(*ring1, Item::zero_item()));
                    let cost = player_war.get_total_cost();
                    if full_fight_won_by_player(player_war.clone(), boss.clone()) {
                        if cost < current_min_cost {
                            current_min_cost = cost;
                        }
                    } else {
                        if cost > current_max_cost {
                            current_max_cost = cost;
                        }
                    }
                    for ring2 in rings.iter() {
                        if ring1 == ring2 {
                            continue;
                        }
                        let mut player_warr = player_war.clone();
                        player_warr.add_rings(&ItemEnum::Ring(*ring1, *ring2));
                        let cost = player_warr.get_total_cost();
                        if full_fight_won_by_player(player_warr.clone(), boss.clone()) {
                            if cost < current_min_cost {
                                current_min_cost = cost;
                            }
                        } else {
                            if cost > current_max_cost {
                                current_max_cost = cost;
                            }
                        }
                    }
                }
            }
        }
        println!("Min score: {}", current_min_cost);
        assert_eq!(current_min_cost, result_1);
        println!("Max score: {}", current_max_cost);
        assert_eq!(current_max_cost, result_2);
    }
    Ok(())
}
