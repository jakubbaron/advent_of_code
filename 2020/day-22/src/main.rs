use std::collections::VecDeque;
use std::io::{self};

struct Player {
    deck: VecDeque<usize>,
}

impl Player {
    fn new(deck: VecDeque<usize>) -> Player {
        Player{deck}
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
        self.deck.iter().rev().enumerate().fold(0, |acc, (i, val)| acc + ((i+1) * val))
    }
}

fn main() -> io::Result<()> {
    let files_results = vec![("test.txt", 306, 1), ("input.txt", 1, 1)];
    for (f, result_1, result_2) in files_results.into_iter() {
        println!("{}", f);
        let file_content: Vec<String> = std::fs::read_to_string(f)?
            .lines()
            .map(|x| x.to_string())
            .collect();
        let mut vec: VecDeque<usize> = VecDeque::new();
        let mut i = 1;
        while i < file_content.len() && !file_content[i].is_empty(){
            let tmp = file_content[i].parse::<usize>().unwrap();
            vec.push_back(tmp);
            i += 1;
        }
        let mut player_1 = Player::new(vec);
        let mut vec: VecDeque<usize> = VecDeque::new();

        i += 2;
        while i< file_content.len() && !file_content[i].is_empty() {
            let tmp = file_content[i].parse::<usize>().unwrap();
            vec.push_back(tmp);
            i += 1;
        }
        let mut player_2 = Player::new(vec);
        while player_1.has_cards() && player_2.has_cards() {
            println!("player_1 {:?}", player_1.deck);
            println!("player_2 {:?}", player_2.deck);
            println!("");

            let card_1 = player_1.draw();
            let card_2 = player_2.draw();
            if card_1 > card_2 {
                player_1.push_cards(card_1, card_2);
            } else {
                player_2.push_cards(card_2, card_1);
            }
        }
        println!("player_1 {:?}", player_1.deck);
        println!("player_2 {:?}", player_2.deck);
        println!("");
        let score = if player_1.has_cards() {
            player_1.get_score()
        } else {
            player_2.get_score()
        };

        assert_eq!(score, result_1);
    }
    Ok(())
}
