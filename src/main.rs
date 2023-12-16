use fancy_regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let content = fs::read_to_string("./input.txt").expect("File should exist.");
    let card_regex = Regex::new(r"(Card\s+\d+\:)").unwrap();
    let mut total_points: u32 = 0;
    let content = content
        .lines()
        .map(|line| card_regex.replace(line, "").trim().to_string())
        .map(split_card)
        .collect::<Vec<Vec<Vec<u32>>>>();
    let mut cards: HashMap<usize, u32> = HashMap::from_iter((1..=content.len()).map(|id| (id, 1u32)));

    for (card_id, card) in content.iter().enumerate() {
        let winner_numbers: HashSet<u32> = HashSet::from_iter(card[0].clone());
        let player_numbers: HashSet<u32> = HashSet::from_iter(card[1].clone());
        let matching_numbers = winner_numbers.intersection(&player_numbers).count();

        if matching_numbers > 0 {
            for id in (2 + card_id) ..= (matching_numbers + card_id + 1) {
                let cloned_cards = cards.clone();
                let increment = cloned_cards.get(&card_id.saturating_add(1)).unwrap();
                
                cards.entry(id).and_modify(|counter| *counter += *increment);
            }

            total_points += 2u32.pow(matching_numbers.saturating_sub(1) as u32);
        }
    }

    println!("Total Cards: {}", cards.values().sum::<u32>());
    println!("Total card points: {}", total_points)
}

fn split_card(card: String) -> Vec<Vec<u32>> {
    card.split('|')
        .map(|numbers| numbers.trim().to_string())
        .map(|numbers_string| {
            numbers_string
                .split(' ')
                .collect::<Vec<&str>>()
                .iter()
                .filter(|value| !value.is_empty())
                .map(|number| match number.trim().parse() {
                    Ok(number) => number,
                    Err(_) => panic!("Card should only contain numbers"),
                })
                .collect()
        })
        .collect()
}
