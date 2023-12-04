use std::collections::HashSet;

fn main() {
    let mut res : u32 = 0;
    for card in std::fs::read_to_string("test.txt").unwrap().lines() {
        let card = &card.split([':', '|']).collect::<Vec<&str>>();
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

        let matched_numbers: Vec<&u32> = numbers
            .iter()
            .filter(|n| winning_numbers.contains(n))
            .collect();

        if matched_numbers.is_empty() {
            continue
        }

        res += 1 << (matched_numbers.len() - 1);
    }

    println!("result is {}", res);
}
