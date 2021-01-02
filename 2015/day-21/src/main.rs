use std::io::{self};
use std::collections::HashMap;
use day_21::{Character, ItemEnum, Item, get_weapons_armors_rings, full_fight_won_by_player};

fn main() -> io::Result<()> {
    let files_results = vec![
        ("input.txt", 1, 1)
    ];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("File: {}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let mut values_map: HashMap<&str, i64> = HashMap::new();
        for line in file_content.iter() {
            let splitted:Vec<&str> = line.split(": ").collect();
            values_map.insert(splitted[0], splitted[1].parse::<i64>().unwrap());
        }
        let mut boss = Character::from_map(&values_map);
        let mut player = Character::new(100, 0, 0);
        let weapons_content: Vec<String> = std::fs::read_to_string("items.txt")?
            .lines()
            .map(|x| x.to_string())
            .collect();

        let (weapons, armors, rings) = get_weapons_armors_rings(&weapons_content);
        let mut sorted_rings = rings.to_vec();
        sorted_rings.sort_by_key(|w| w.cost);
        let (last_ring, rest_of_rings) = sorted_rings.split_last().unwrap();
        let max_cost = weapons.iter().max_by_key(|w| w.cost).unwrap().cost + armors.iter().max_by_key(|a| a.cost).unwrap().cost + last_ring.cost + rest_of_rings.last().unwrap().cost;
        println!("Max cost: {}", max_cost);
        let mut current_min_cost = i64::MAX;
        for weapon in weapons.iter() {
            let mut player_w = player.clone();
            player_w.add_weapon(&ItemEnum::Weapon(*weapon));
            if full_fight_won_by_player(player_w.clone(), boss) {
                let tmp = player_w.get_total_cost();
                if tmp < current_min_cost {
                    current_min_cost = tmp;
                }
            }
            for armor in armors.iter() {
                let mut player_wa = player_w.clone();
                player_wa.add_armor(&ItemEnum::Armor(*armor));
                if full_fight_won_by_player(player_wa.clone(), boss) {
                    let tmp = player_wa.get_total_cost();
                    if tmp < current_min_cost {
                        current_min_cost = tmp;
                    }
                }
                for ring1 in rings.iter() {
                    let mut player_war = player_wa.clone();
                    player_war.add_rings(&ItemEnum::Ring(*ring1, Item::zero_item()));
                    if full_fight_won_by_player(player_war.clone(), boss) {
                        let tmp = player_war.get_total_cost();
                        if tmp < current_min_cost {
                            current_min_cost = tmp;
                        }
                    }
                    for ring2 in rings.iter() {
                        if ring1 == ring2 {
                            continue
                        }
                        let mut player_warr = player_war.clone();
                        player_warr.add_rings(&ItemEnum::Ring(*ring1, *ring2));
                        if full_fight_won_by_player(player_warr.clone(), boss) {
                            let tmp = player_warr.get_total_cost();
                            if tmp < current_min_cost {
                                current_min_cost = tmp;
                            }
                        }
                    }
                }
            }
        }
        println!("Min score: {}", current_min_cost);
    }
    Ok(())
}
