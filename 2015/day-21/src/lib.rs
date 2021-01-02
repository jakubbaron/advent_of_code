use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub enum ItemEnum {
    Armor(Item),
    Weapon(Item),
    Ring(Item, Item),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item {
    damage: i64,
    armor: i64,
    pub cost: i64,
}

impl Item {
    pub fn new(damage: i64, armor: i64, cost: i64) -> Item {
        Item {
            damage,
            armor,
            cost,
        }
    }

    pub fn zero_item() -> Item {
        Item::new(0, 0, 0)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Character {
    hitpoints: i64,
    damage: i64,
    armor: i64,
    weapon_wield: Item,
    armor_wield: Item,
    rings: (Item, Item),
}

impl Character {
    pub fn new(hitpoints: i64, damage: i64, armor: i64) -> Character {
        let weapon_wield = Item::zero_item();
        let armor_wield = Item::zero_item();
        let rings = (Item::zero_item(), Item::zero_item());
        Character {
            hitpoints,
            damage,
            armor,
            weapon_wield,
            armor_wield,
            rings,
        }
    }

    pub fn from_map(values_map: &HashMap<&str, i64>) -> Character {
        let weapon_wield = Item::zero_item();
        let armor_wield = Item::zero_item();
        let rings = (Item::zero_item(), Item::zero_item());
        Character {
            hitpoints: *values_map.get(&"Hit Points").unwrap(),
            damage: *values_map.get(&"Damage").unwrap(),
            armor: *values_map.get(&"Armor").unwrap(),
            weapon_wield,
            armor_wield,
            rings,
        }
    }
    fn get_additional_damage(&self) -> i64 {
        self.weapon_wield.damage + self.rings.0.damage + self.rings.1.damage
    }

    fn get_damage(&self) -> i64 {
        self.damage + self.get_additional_damage()
    }

    fn get_additional_armor(&self) -> i64 {
        self.armor_wield.armor + self.rings.0.armor + self.rings.1.armor
    }

    fn get_armor(&self) -> i64 {
        self.armor + self.get_additional_armor()
    }

    fn deal_damage_with(&mut self, other: &Character) {
        let damage_to_receive = std::cmp::max(other.get_damage() - self.get_armor(), 1);
        self.hitpoints -= damage_to_receive;
    }

    pub fn is_defeated(&self) -> bool {
        self.hitpoints <= 0
    }

    pub fn add_weapon(&mut self, new_weapon: &ItemEnum) {
        match new_weapon {
            ItemEnum::Weapon(new_weapon) => self.weapon_wield = *new_weapon,
            _ => {
                println!("Trying to assing a NonWeapon as a Weapon");
            }
        };
    }

    pub fn add_armor(&mut self, new_armor: &ItemEnum) {
        match new_armor {
            ItemEnum::Armor(new_armor) => self.armor_wield = *new_armor,
            _ => {
                println!("Trying to assing a NonArmor as an Armor");
            }
        };
    }

    pub fn add_rings(&mut self, new_rings: &ItemEnum) {
        match new_rings {
            ItemEnum::Ring(r1, r2) => self.rings = (*r1, *r2),
            _ => {
                println!("Trying to assing a NonRing as a Ring");
            }
        }
    }

    pub fn get_total_cost(&self) -> i64 {
        self.weapon_wield.cost + self.armor_wield.cost + self.rings.0.cost + self.rings.1.cost
    }
}

pub fn fight_round(player: &mut Character, boss: &mut Character) {
    boss.deal_damage_with(&player);
    // println!("The player deals {} damage; the boss goes down to {} hit points.", player.get_damage() - boss.get_armor(), boss.hitpoints);
    if boss.is_defeated() {
        return;
    }
    player.deal_damage_with(&boss);
    // println!("The boss deals {} damage; the player goes down to {} hit points.", boss.get_damage() - player.get_armor(), player.hitpoints);
}

pub fn full_fight_won_by_player(mut player: Character, mut boss: Character) -> bool {
    while !boss.is_defeated() && !player.is_defeated() {
        fight_round(&mut player, &mut boss);
    }
    boss.is_defeated()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deal_damage_with() {
        let mut player = Character::new(8, 5, 5);
        let mut boss = Character::new(12, 7, 2);
        player.deal_damage_with(&mut boss);
        assert_eq!(player.hitpoints, 6);
        assert_eq!(player.damage, 5);
        assert_eq!(player.armor, 5);
    }

    #[test]
    fn test_deal_damage_with_more_armor_than_dmg() {
        let mut player = Character::new(8, 5, 5);
        let mut boss = Character::new(12, 7, 200);
        boss.deal_damage_with(&mut player);
        assert_eq!(boss.hitpoints, 11);
    }

    #[test]
    fn test_fight_round() {
        let mut player = Character::new(8, 5, 5);
        let mut boss = Character::new(12, 7, 2);
        fight_round(&mut player, &mut boss);
        assert_eq!(player.hitpoints, 6);
        assert_eq!(boss.hitpoints, 9);
    }

    #[test]
    fn test_is_defeated_after_fight() {
        let mut player = Character::new(8, 5, 0);
        let mut boss = Character::new(12, 8, 2);
        fight_round(&mut player, &mut boss);
        assert_eq!(player.hitpoints, 0);
        assert!(player.is_defeated());
    }

    #[test]
    fn test_is_defeated() {
        let mut player = Character::new(-1, 5, 0);
        assert!(player.is_defeated());
    }

    #[test]
    fn test_add_weapon() {
        let sword = ItemEnum::Weapon(Item::new(10, 0, 0));
        let mut player = Character::new(8, 5, 5);
        player.add_weapon(&sword);
        assert_eq!(player.get_damage(), 10 + 5);
    }

    #[test]
    fn test_add_second_weapon_overrides_first_one() {
        let sword_1 = ItemEnum::Weapon(Item::new(10, 0, 0));
        let sword_2 = ItemEnum::Weapon(Item::new(15, 0, 0));
        let mut player = Character::new(8, 5, 5);
        player.add_weapon(&sword_1);
        assert_eq!(player.get_damage(), 10 + 5);
        player.add_weapon(&sword_2);
        assert_eq!(player.get_damage(), 15 + 5);
    }

    #[test]
    fn test_add_weapon_adding_armor_doesnt_set_as_weapon() {
        let sword = ItemEnum::Weapon(Item::new(10, 0, 0));
        let armor = ItemEnum::Armor(Item::new(0, 10, 0));
        let mut player = Character::new(8, 5, 5);
        player.add_weapon(&sword);
        assert_eq!(player.get_damage(), 10 + 5);
        player.add_weapon(&armor);
        assert_eq!(player.get_damage(), 10 + 5);
    }

    #[test]
    fn test_add_armor() {
        let armor = ItemEnum::Armor(Item::new(0, 10, 0));
        let mut player = Character::new(8, 5, 5);
        player.add_armor(&armor);
        assert_eq!(player.get_armor(), 10 + 5);
    }

    #[test]
    fn test_add_second_armor_overrides_first_one() {
        let armor_1 = ItemEnum::Armor(Item::new(0, 10, 0));
        let armor_2 = ItemEnum::Armor(Item::new(0, 15, 0));
        let mut player = Character::new(8, 5, 5);
        player.add_armor(&armor_1);
        assert_eq!(player.get_armor(), 10 + 5);
        player.add_armor(&armor_2);
        assert_eq!(player.get_armor(), 15 + 5);
    }

    #[test]
    fn test_add_armor_adding_weapon_doesnt_set_as_armor() {
        let sword = ItemEnum::Weapon(Item::new(10, 0, 0));
        let armor = ItemEnum::Armor(Item::new(0, 10, 0));
        let mut player = Character::new(8, 5, 5);
        player.add_armor(&armor);
        assert_eq!(player.get_armor(), 10 + 5);
        player.add_armor(&sword);
        assert_eq!(player.get_armor(), 10 + 5);
    }

    #[test]
    fn test_full_fight() {
        let mut boss = Character::new(12, 7, 2);
        let mut player = Character::new(8, 5, 5);
        assert!(full_fight_won_by_player(&mut player, &mut boss));
        assert!(boss.is_defeated());
        assert!(!player.is_defeated());
    }

    #[test]
    fn test_get_total_cost() {
        let mut player = Character::new(8, 5, 5);
        let sword = ItemEnum::Weapon(Item::new(10, 0, 10));
        let armor = ItemEnum::Armor(Item::new(0, 10, 20));
        let ring = Item::new(3, 0, 5);
        let rings = ItemEnum::Ring(ring, ring);
        player.add_weapon(&sword);
        player.add_armor(&armor);
        player.add_rings(&rings);
        assert_eq!(player.get_total_cost(), 10 + 20 + 2 * 5);
    }
}

fn parse_item(line: &String) -> Item {
    let splitted: Vec<&str> = line.split(" ").filter(|x| !x.is_empty()).collect();
    let v: Vec<i64> = splitted[1..4]
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    Item::new(v[1], v[2], v[0])
}

pub fn get_weapons_armors_rings(
    weapons_content: &Vec<String>,
) -> (Vec<Item>, Vec<Item>, Vec<Item>) {
    enum Parser {
        Weapons,
        Armors,
        Rings,
    };
    let mut mode = Parser::Weapons;
    let mut weapons: Vec<Item> = Vec::new();
    let mut armors: Vec<Item> = Vec::new();
    let mut rings: Vec<Item> = Vec::new();
    for line in weapons_content.iter() {
        match mode {
            Parser::Weapons => {
                if line.starts_with("Weapons") {
                    continue;
                }
                if line.is_empty() {
                    mode = Parser::Armors;
                    continue;
                }
                weapons.push(parse_item(&line));
            }
            Parser::Armors => {
                if line.starts_with("Armor") {
                    continue;
                }
                if line.is_empty() {
                    mode = Parser::Rings;
                    continue;
                }
                armors.push(parse_item(&line));
            }
            Parser::Rings => {
                if line.starts_with("Rings") {
                    continue;
                }
                if line.is_empty() {
                    break;
                }
                rings.push(parse_item(&line));
            }
        };
    }
    (weapons, armors, rings)
}
