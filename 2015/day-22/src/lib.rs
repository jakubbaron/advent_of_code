use std::collections::{HashMap, HashSet};

#[derive(Hash, PartialEq, Eq)]
pub enum SpellEnum {
    MagicMissle,
    Drain,
    Shield,
    Poison,
    Recharge,
}

#[derive(Clone)]
pub struct Character {
    pub hitpoints: i64,
    pub damage: i64,
    pub armor: i64,
    pub mana: i64,
    pub effects: Vec<Spell>,
    pub total_mana_used: i64,
}

impl Character {
    pub fn is_defeated(&self) -> bool {
        self.hitpoints <= 0
    }

    pub fn new(hitpoints: i64, damage: i64, mana: i64) -> Character {
        Character {
            hitpoints,
            damage,
            mana,
            armor: 0,
            effects: Vec::new(),
            total_mana_used: 0,
        }
    }

    pub fn from_map(values_map: &HashMap<&str, i64>) -> Character {
        let hitpoints = *values_map.get("Hit Points").unwrap();
        let damage = *values_map.get("Damage").unwrap();
        Character::new(hitpoints, damage, 0)
    }

    pub fn get_armor(&self) -> i64 {
        self.armor + self.effects.iter().fold(0, |acc, val| acc + val.armor)
    }

    pub fn apply_effects(&mut self, boss: &mut Character) {
        let mut to_keep: Vec<Spell> = Vec::new();
        for effect in self.effects.iter_mut() {
            effect.turns -= 1;
            if effect.turns > 0 {
                println!("{}'s timer is now at {}", effect.name, effect.turns);
            } else {
                println!("{}'s timer wears off", effect.name);
            }
            boss.hitpoints -= effect.damage;
            self.mana += effect.mana_gain;
            if effect.turns > 0 {
                to_keep.push(effect.clone());
            }
        }
        self.effects = to_keep;
    }

    pub fn cast_spell(&mut self, spell: &Spell, boss: &mut Character) {
        if self.mana < spell.mana_cost {
            panic!("Not enough mana to cast {}", spell.name);
        }
        self.mana -= spell.mana_cost;
        self.total_mana_used += spell.mana_cost;
        if spell.turns == 0 {
            boss.hitpoints -= spell.damage;
            self.hitpoints += spell.heals_for;
        } else {
            self.effects.push(spell.clone());
        }
    }

    pub fn attack(&mut self, other: &mut Character) {
        let dmg_to_deal = std::cmp::max(1, self.damage - other.get_armor());
        println!("Boss deals {} dmg", dmg_to_deal);
        other.hitpoints -= dmg_to_deal;
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Spell {
    pub name: String,
    damage: i64,
    armor: i64,
    heals_for: i64,
    pub mana_cost: i64,
    mana_gain: i64,
    turns: i64,
}

impl Spell {
    fn new_spell(name: &str, damage: i64, mana_cost: i64, heals_for: i64) -> Spell {
        Spell {
            name: name.to_string(),
            damage,
            mana_cost,
            heals_for,
            mana_gain: 0,
            turns: 0,
            armor: 0,
        }
    }
    fn new_effect(
        name: &str,
        damage: i64,
        mana_cost: i64,
        mana_gain: i64,
        armor: i64,
        turns: i64,
    ) -> Spell {
        Spell {
            name: name.to_string(),
            damage,
            mana_cost,
            mana_gain,
            armor,
            turns,
            heals_for: 0,
        }
    }
}

pub fn get_all_spells() -> HashMap<SpellEnum, Spell> {
    vec![
        // Magic Missle
        (
            SpellEnum::MagicMissle,
            Spell::new_spell("Magic Missle", 4, 53, 0),
        ),
        // Drain
        (SpellEnum::Drain, Spell::new_spell("Drain", 2, 73, 2)),
        // Shield
        (
            SpellEnum::Shield,
            Spell::new_effect("Shield", 0, 113, 0, 7, 6),
        ),
        // Poison
        (
            SpellEnum::Poison,
            Spell::new_effect("Poison", 3, 173, 0, 0, 6),
        ),
        // Recharge
        (
            SpellEnum::Recharge,
            Spell::new_effect("Recharge", 0, 229, 101, 0, 5),
        ),
    ]
    .into_iter()
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attack() {
        let mut boss = Character::new(50, 5, 500);
        let mut player = Character::new(50, 0, 500);
        boss.attack(&mut player);
        assert_eq!(player.hitpoints, 45);
    }

    #[test]
    fn test_effect_shield() {
        let mut boss = Character::new(50, 5, 500);
        let mut player = Character::new(50, 0, 500);
        player.cast_spell(*get_all_spells().get(&SpellEnum::Shield).unwrap());
        boss.attack(&mut player);
        assert_eq!(player.hitpoints, 49);
    }

    #[test]
    fn test_effect_wears_off() {
        let mut boss = Character::new(50, 5, 500);
        let mut player = Character::new(50, 0, 500);
        let shield_spell = *get_all_spells().get(&SpellEnum::Shield).unwrap();
        let turns = shield_spell.turns;
        player.cast_spell(shield_spell);
        for _ in 0..turns {
            player.apply_effects(&mut boss);
        }
        boss.attack(&mut player);
        assert_eq!(player.hitpoints, 45);
    }
    #[test]
    fn test_recharge() {
        let mut boss = Character::new(50, 5, 500);
        let mut player = Character::new(50, 0, 500);
        let recharge_spell = *get_all_spells().get(&SpellEnum::Recharge).unwrap();
        let Spell {
            mana_gain,
            mana_cost,
            ..
        } = recharge_spell.clone();
        player.cast_spell(recharge_spell);
        player.apply_effects(&mut boss);
        assert_eq!(player.mana, 500 + mana_gain - mana_cost);
        player.apply_effects(&mut boss);
        assert_eq!(player.mana, 500 + 2 * mana_gain - mana_cost);
    }
}
