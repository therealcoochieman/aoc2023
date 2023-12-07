use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    process,
    slice::Iter,
};

static COMPARE: [(char, u32); 5] =
    [('A', 14), ('K', 13), ('Q', 12), ('J', 1), ('T', 10)];

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    pub fn from_histogram(histogram: &[u8]) -> Self {
        match histogram[0] {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 if histogram[1] == 2 => HandType::FullHouse,
            3 => HandType::ThreeOfAKind,
            2 if histogram[1] == 2 => HandType::TwoPair,
            2 => HandType::OnePair,
            1 => HandType::HighCard,
            e => {
                eprintln!("most cards number is {e}???");
                process::exit(1)
            }
        }
    }

    pub fn ascending_iter() -> Iter<'static, HandType> {
        static TYPES: [HandType; 7] = [
            HandType::HighCard,
            HandType::OnePair,
            HandType::TwoPair,
            HandType::ThreeOfAKind,
            HandType::FullHouse,
            HandType::FourOfAKind,
            HandType::FiveOfAKind,
        ];

        TYPES.iter()
    }
}

struct Hand {
    pub r#type: HandType,
    pub hand: String,
    pub bid: u32,
}

impl Hand {
    pub fn new(line: &[&str]) -> Self {
        let hand = line[0];
        let mut histogram: HashMap<char, u8> = HashMap::new();
        hand.chars().for_each(|c| {
            *histogram.entry(c).or_default() += 1;
        });

        let mut jokers = 0;
        if let Some(joker) = histogram.get_mut(&'J') {
            jokers = *joker;
            *joker = 0;
        }

        let mut histogram: Vec<u8> =
            Vec::from_iter(histogram.values().copied());
        histogram.sort_by(|a, b| b.cmp(a));
        histogram[0] += jokers;
        let r#type = HandType::from_histogram(&histogram);
        Hand {
            r#type,
            hand: line[0].into(),
            bid: line[1].parse().unwrap(),
        }
    }

    fn card_to_num(char: char, compare_map: &HashMap<char, u32>) -> u32 {
        if char.is_ascii_digit() {
            char.to_digit(10).unwrap()
        } else {
            compare_map[&char]
        }
    }

    pub fn cmp(
        &self,
        other: &Hand,
        compare_map: &HashMap<char, u32>,
    ) -> Ordering {
        let hand = self.hand.chars().enumerate();
        let other_hand = other.hand.chars().collect::<Vec<char>>();
        for (index, card) in hand {
            let card = Hand::card_to_num(card, compare_map);
            let other_card = Hand::card_to_num(other_hand[index], compare_map);
            let cmp = card.cmp(&other_card);
            if cmp != Ordering::Equal {
                return cmp;
            }
        }

        Ordering::Equal
    }
}

fn main() {
    let file = File::open("test.txt").unwrap();
    let reader = BufReader::new(&file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let mut hands: HashMap<HandType, Vec<Hand>> = HashMap::new();
    for line in &lines {
        let line = line.split(' ').collect::<Vec<&str>>();
        let hand = Hand::new(&line);
        hands.entry(hand.r#type).or_default().push(hand);
    }
    let mut rank = 1;
    let mut total_winnings = 0;
    let compare_map: HashMap<char, u32> = COMPARE.iter().copied().collect();
    for r#type in HandType::ascending_iter() {
        let hands = hands.get_mut(r#type);
        if hands.is_none() {
            continue;
        }
        let hands = hands.unwrap();
        hands.sort_by(|a, b| a.cmp(b, &compare_map));
        for hand in hands {
            total_winnings += hand.bid * rank;
            rank += 1;
        }
    }

    println!("total winnings are {total_winnings}");
}
