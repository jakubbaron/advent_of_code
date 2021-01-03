use day_22::{get_all_spells, Character, Spell, SpellEnum};
use rand::distributions::{Distribution, Uniform};
use std::collections::HashMap;
use std::io::{self};

fn get_next_spell(p: &Character, spells: &[&Spell]) -> Option<Spell> {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..spells.len());
    if p.mana < 53 {
        return None;
    }
    loop {
        let i: usize = die.sample(&mut rng);
        let s = spells[i];
        if !p.effects.iter().any(|x| x.name == s.name) && p.mana >= s.mana_cost {
            return Some(s.clone());
        }
    }
}

fn do_battle(
    mut player: Character,
    mut boss: Character,
    spells: &[&Spell],
    hard_level: bool,
) -> i64 {
    while !boss.is_defeated() && !player.is_defeated() {
        println!("");
        println!("--Player turn --");
        println!(
            "Player has {} hit points, {} armor, {} mana",
            player.hitpoints,
            player.get_armor(),
            player.mana
        );
        println!("Boss has {} hit points", boss.hitpoints);
        if hard_level {
            player.hitpoints -= 1;
            if player.is_defeated() {
                return i64::MAX;
            }
        }
        player.apply_effects(&mut boss);
        let some_spell = match get_next_spell(&player, &spells[..]) {
            Some(s) => s,
            None => {
                return i64::MAX;
            }
        };
        println!("Player casts {:?}", some_spell.name);
        player.cast_spell(&some_spell, &mut boss);
        println!("");
        println!("--Boss turn --");
        println!(
            "Player has {} hit points, {} armor, {} mana",
            player.hitpoints,
            player.get_armor(),
            player.mana
        );
        println!("Boss has {} hit points", boss.hitpoints);
        player.apply_effects(&mut boss);
        if boss.is_defeated() {
            println!("Boss is defeated");
            println!("Mana used: {}", player.total_mana_used);
            return player.total_mana_used;
        }
        boss.attack(&mut player);
    }
    i64::MAX
}

fn main() -> io::Result<()> {
    let files_results = vec![("input.txt", 900, 1_216)];
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
        let bo = Character::from_map(&values_map);
        let pl = Character::new(50, 0, 500);

        let m = get_all_spells();
        let r = m.get(&SpellEnum::Recharge).unwrap();
        let s = m.get(&SpellEnum::Shield).unwrap();
        let d = m.get(&SpellEnum::Drain).unwrap();
        let p = m.get(&SpellEnum::Poison).unwrap();
        let mm = m.get(&SpellEnum::MagicMissle).unwrap();
        let spells = vec![r, s, d, p, mm];
        let mut min_mana_used = i64::MAX;
        for i in 0..3_000 {
            println!("");
            println!("NEW BATTLE {}", i);
            min_mana_used = std::cmp::min(
                do_battle(pl.clone(), bo.clone(), &spells, false),
                min_mana_used,
            );
        }
        println!("Min mana used: {}", min_mana_used);
        assert_eq!(min_mana_used, result_1);

        let mut min_mana_used = i64::MAX;

        for i in 0..9_000 {
            println!("");
            println!("NEW BATTLE {}", i);
            min_mana_used = std::cmp::min(
                do_battle(pl.clone(), bo.clone(), &spells, true),
                min_mana_used,
            );
        }
        println!("Min mana used, level hard: {}", min_mana_used);
        assert_eq!(min_mana_used, result_2);
    }
    Ok(())
}
