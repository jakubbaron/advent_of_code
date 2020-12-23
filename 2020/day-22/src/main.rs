use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Player {
    deck: VecDeque<usize>,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum PlayerEnum {
    Player1,
    Player2,
}

impl Player {
    fn new(deck: VecDeque<usize>) -> Player {
        Player { deck }
    }
    fn draw(&mut self) -> usize {
        self.deck.pop_front().unwrap()
    }
    fn has_cards(&self) -> bool {
        !self.deck.is_empty()
    }
    fn push_cards(&mut self, card_1: usize, card_2: usize) {
        self.deck.push_back(card_1);
        self.deck.push_back(card_2);
    }
    fn get_score(&self) -> usize {
        self.deck
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, val)| acc + ((i + 1) * val))
    }
}

struct Game {
    player_1: Player,
    player_2: Player,
}

impl Game {
    fn new(player_1: &Player, player_2: &Player) -> Game {
        Game {player_1: player_1.clone(), player_2: player_2.clone()}
    }

    fn play(&mut self) {
        while self.player_1.has_cards() && self.player_2.has_cards() {
            println!("player_1 {:?}", self.player_1.deck);
            println!("player_2 {:?}", self.player_2.deck);
            println!("");

            let card_1 = self.player_1.draw();
            let card_2 = self.player_2.draw();
            if card_1 > card_2 {
                self.player_1.push_cards(card_1, card_2);
            } else {
                self.player_2.push_cards(card_2, card_1);
            }
        }
    }

    fn get_score(&self) -> Option<usize> {
        if self.player_1.has_cards() && self.player_2.has_cards() {
            println!("The game hasn't been played, no score");
            return None;
        }
        if self.player_1.has_cards() {
            Some(self.player_1.get_score())
        } else {
            Some(self.player_2.get_score())
        }
    }
}

struct Game2 {
    previous_cards: HashMap<PlayerEnum, HashSet<u64>>,
    player_1: Player,
    player_2: Player,
    game_id: usize,
}

static mut GAME_ID:usize = 1_usize;

impl Game2 {
    fn new(player_1: &Player, player_2: &Player) -> Game2 {
        let game_id;
        unsafe {
            println!("NEW GAME {}", GAME_ID);
            game_id = GAME_ID.clone();
            GAME_ID += 1;
        }
        Game2{player_1: player_1.clone(), player_2: player_2.clone(), previous_cards:HashMap::new(), game_id}
    }

    fn _same_decks(&mut self, p: PlayerEnum) -> bool {
        let deck = match p {
            PlayerEnum::Player1 => &self.player_1.deck,
            PlayerEnum::Player2 => &self.player_2.deck,
        };
        let mut hasher = DefaultHasher::new();
        deck.hash(&mut hasher);
        let hash = hasher.finish();
        let prev_cards = self.previous_cards.entry(p).or_insert_with(HashSet::new);
        if prev_cards.contains(&hash) {
            return true;
        }
        prev_cards.insert(hash);
        return false;
    }

    fn same_decks(&mut self) -> bool {
        self._same_decks(PlayerEnum::Player1) || self._same_decks(PlayerEnum::Player2)
    }

    fn should_recurse(&self, card_1: usize, card_2: usize) -> bool {
        card_1 <= self.player_1.deck.len() && card_2 <= self.player_2.deck.len()
    }

    fn play(&mut self) -> PlayerEnum {
        let mut round = 0;
        loop {
            round += 1;
            println!("--- Round {} (Game {}) --", round, self.game_id);
            println!("player_1 {:?}", self.player_1.deck);
            println!("player_2 {:?}", self.player_2.deck);
            println!("");

            if self.same_decks() {
                println!("Same deck cards, Player1 wins");
                return PlayerEnum::Player1;
            }
            if !self.player_1.has_cards() {
                println!("Player1 no cards, Player2 wins");
                return PlayerEnum::Player2;
            } else if !self.player_2.has_cards() {
                println!("Player2 no cards, Player1 wins");
                return PlayerEnum::Player1;
            }

            let card_1 = self.player_1.draw();
            let card_2 = self.player_2.draw();
            if self.should_recurse(card_1, card_2) {
                if !self.player_1.has_cards() || !self.player_2.has_cards() {
                    println!("Cannot recurse, no more cards");
                    if card_1 > card_2 {
                        println!("Player1 wins");
                        return PlayerEnum::Player1;
                    } else {
                        println!("Player2 wins");
                        return PlayerEnum::Player2;
                    }
                }
                let mut new_player_1 = self.player_1.clone();
                while new_player_1.deck.len() > card_1 {
                    new_player_1.deck.pop_back();
                }
                let mut new_player_2 = self.player_2.clone();
                while new_player_2.deck.len() > card_2 {
                    new_player_2.deck.pop_back();
                }
                let mut new_game = Game2::new(&new_player_1, &new_player_2);
                match new_game.play() {
                    PlayerEnum::Player1 => {
                        self.player_1.push_cards(card_1, card_2);
                    },
                    PlayerEnum::Player2 => {
                        self.player_2.push_cards(card_2, card_1);
                    },
                }
            } else {
                if card_1 > card_2 {
                    self.player_1.push_cards(card_1, card_2);
                } else {
                    self.player_2.push_cards(card_2, card_1);
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    let files_results = vec![("test.txt", 306, 291), ("input.txt", 35013, 32806)];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("{}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let mut i = 1;
        while i < file_content.len() && !file_content[i].is_empty() {
            i += 1;
        }
        let player_1 = Player::new(
            file_content[1..i]
                .iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect(),
        );
        let player_2 = Player::new(
            file_content[i + 2..file_content.len()]
                .iter()
                .map(|x| x.parse::<usize>().unwrap())
                .collect(),
        );

        let mut game = Game::new(&player_1,&player_2);
        game.play();
        let score = game.get_score().unwrap();

        assert_eq!(score, result_1);
        let mut game2 = Game2::new(&player_1, &player_2);

        let score_2 = match game2.play() {
            PlayerEnum::Player1 => game2.player_1.get_score(),
            PlayerEnum::Player2 => game2.player_2.get_score(),
        };
        assert_eq!(score_2, result_2);
    }
    Ok(())
}
