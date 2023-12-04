use std::collections::{HashMap, HashSet};
struct Card {
    pub index: usize,
    pub winning_numbers: HashSet<u32>,
    pub numbers: Vec<u32>,
}

impl Card {
    pub fn match_numbers(&self) -> Vec<&u32> {
        self.numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .collect()
    }

    pub fn get_res(&self) -> u32 {
        let matched_numbers = &self.match_numbers();

        if matched_numbers.is_empty() {
            return 0;
        }

        1 << (matched_numbers.len() - 1)
    }

    pub fn from_vec(card: &[&str], index: usize) -> Card {
        let winning_numbers: HashSet<u32> = card[1]
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        let numbers: Vec<u32> = card[2]
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        Card {
            winning_numbers,
            numbers,
            index,
        }
    }
}

fn main() {
    let mut cards: Vec<Card> = Vec::new();
    let mut copy_map: HashMap<usize, u64> = HashMap::new();
    std::fs::read_to_string("test.txt")
        .unwrap()
        .lines()
        .enumerate()
        .for_each(|(index, card)| {
            let card = &card.split([':', '|']).collect::<Vec<&str>>();
            cards.push(Card::from_vec(card, index + 1));
            copy_map.insert(index + 1, 1);
        });

    cards.iter().for_each(|card| {
        let copies = card.match_numbers().len();
        for i in card.index + 1..=card.index + copies {
            *copy_map.get_mut(&i).unwrap() += copy_map[&card.index];
        }
    });

    println!("result is {}", copy_map.values().sum::<u64>());
}
