use std::fs;
struct Card {
    matches: u32,
}

pub fn main() {
    let content = fs::read_to_string("./src/day4/input.txt").expect("Could not read file");

    let mut cards: Vec<Card> = Vec::new();

    // parse input
    for line in content.lines() {
        let first_split: Vec<&str> = line.split(':').collect();
        if first_split.len() != 2 {
            panic!("Invalid line: {}", line);
        }
        let numbers_part = first_split[1].trim();
        let numbers_split: Vec<&str> = numbers_part.split('|').collect();
        if numbers_split.len() != 2 {
            panic!("Invalid line: {}", line);
        }
        let winning_numbers_str = numbers_split[0].trim();
        let winning_numbers: Vec<u32> = get_numbers_from_text(winning_numbers_str);

        let numbers_str = numbers_split[1].trim();
        let numbers = get_numbers_from_text(numbers_str);

        let mut matches: u32 = 0;
        for number in numbers.iter() {
            if winning_numbers.contains(number) {
                matches += 1;
            }
        }
        let card = Card { matches };
        cards.push(card);
    }

    // task 1
    let mut score: u32 = 0;
    for card in cards.iter() {
        if card.matches > 0 {
            score += 2_u32.pow(card.matches - 1);
        }
    }

    println!("Task 1: {}", score); //18519

    // task 2
    let mut card_copies: Vec<u32> = vec![1; cards.len()];
    for (index, card) in cards.iter().enumerate() {
        let current_card_copies = card_copies[index];
        for card_copy_id in (index + 1)..=(index + card.matches as usize) {
            card_copies[card_copy_id] += current_card_copies;
        }
    }

    let cards_sum: u32 = card_copies.iter().sum();
    println!("Task 2: {}", cards_sum); //11787590
}

fn get_numbers_from_text(text: &str) -> Vec<u32> {
    text.split(' ')
        .map(|n| n.trim().parse::<u32>())
        .filter(|n| n.is_ok())
        .map(|n| n.unwrap())
        .collect()
}
