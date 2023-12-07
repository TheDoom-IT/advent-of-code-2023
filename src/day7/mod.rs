use std::cmp::Ordering;
use std::fs;

#[derive(Debug, Clone)]
struct Hand {
    cards: String,
    bid: u64,
}

#[derive(Debug, Clone)]
struct HandWithType {
    hand: Hand,
    hand_type: Type,
}
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

const CARDS_ORDER: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

const CARDS_WITH_JOKER_ORDER: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

pub fn main() {
    let content = fs::read_to_string("./src/day7/input.txt").expect("Could not read file");

    let mut hands: Vec<Hand> = Vec::new();
    for line in content.lines() {
        let line_split: Vec<&str> = line.split(' ').collect();
        if line_split.len() != 2 {
            panic!("Invalid line")
        }

        let bid_result = line_split[1].parse::<u64>();
        if bid_result.is_err() {
            panic!("Invalid line")
        }

        if line_split[0].len() != 5 {
            panic!("Invalid line")
        }

        hands.push(Hand {
            cards: line_split[0].to_string(),
            bid: bid_result.unwrap(),
        })
    }

    // task 1
    let mut hands_with_type = hands
        .iter()
        .map(|h| define_type_of_hand(h.clone()))
        .collect();

    hands_with_type = sort_hands_with_type(hands_with_type, CARDS_ORDER);

    let score = get_score(&hands_with_type);

    println!("Task 1: {}", score); //254024898

    // task 2
    let mut joker_hands_with_type = hands
        .into_iter()
        .map(|h| define_type_of_hand_with_joker(h))
        .collect();

    joker_hands_with_type = sort_hands_with_type(joker_hands_with_type, CARDS_WITH_JOKER_ORDER);
    let score = get_score(&joker_hands_with_type);

    println!("Task 2: {}", score); //254115617
}

fn get_score(hands: &Vec<HandWithType>) -> u64 {
    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| (index + 1) as u64 * hand.hand.bid)
        .sum::<u64>()
}
fn sort_hands_with_type(mut hands: Vec<HandWithType>, cards: [char; 13]) -> Vec<HandWithType> {
    hands.sort_by(|a, b| {
        if a.hand_type == b.hand_type {
            for (index, card_a) in a.hand.cards.chars().enumerate() {
                let card_b = b.hand.cards.chars().nth(index).unwrap();
                let card_a_order = cards.iter().position(|card| *card == card_a).unwrap();
                let card_b_order = cards.iter().position(|card| *card == card_b).unwrap();
                if card_a_order > card_b_order {
                    return Ordering::Less;
                } else if card_a_order < card_b_order {
                    return Ordering::Greater;
                }
            }
            return Ordering::Equal;
        }

        return a.hand_type.cmp(&b.hand_type).reverse();
    });

    return hands;
}

fn get_type_from_cards_count(cards_count: &Vec<u64>) -> Type {
    if *cards_count == vec![5] {
        Type::FiveOfAKind
    } else if cards_count[..] == [1, 4] {
        Type::FourOfAKind
    } else if cards_count[..] == [2, 3] {
        Type::FullHouse
    } else if cards_count[..] == [1, 1, 3] {
        Type::ThreeOfAKind
    } else if cards_count[..] == [1, 2, 2] {
        Type::TwoPair
    } else if cards_count[..] == [1, 1, 1, 2] {
        Type::OnePair
    } else {
        Type::HighCard
    }
}

fn define_type_of_hand_with_joker(hand: Hand) -> HandWithType {
    let cards: Vec<char> = hand.cards.chars().collect();
    let cards_without_joker: Vec<char> = cards
        .iter()
        .filter(|card| **card != 'J')
        .map(|c| c.clone())
        .collect();

    let joker_count = cards.len() - cards_without_joker.len();

    let mut cards_count: Vec<u64> = count_cards(&cards_without_joker);

    if cards_count.len() == 0 {
        // there are only jokers on the hand
        cards_count.push(joker_count as u64);
    } else {
        // add jokers to the most common card
        let last_item = cards_count.len() - 1;
        cards_count[last_item] += joker_count as u64;
    }

    let hand_type = get_type_from_cards_count(&cards_count);
    HandWithType {
        hand: Hand { ..hand },
        hand_type,
    }
}

fn define_type_of_hand(hand: Hand) -> HandWithType {
    let cards: Vec<char> = hand.cards.chars().collect();
    let cards_count: Vec<u64> = count_cards(&cards);

    let hand_type = get_type_from_cards_count(&cards_count);

    HandWithType {
        hand: Hand { ..hand },
        hand_type,
    }
}

fn count_cards(cards: &Vec<char>) -> Vec<u64> {
    let mut unique_cards = cards.clone();
    unique_cards.sort();
    unique_cards.dedup();

    let mut cards_count: Vec<u64> = Vec::new();
    for card in unique_cards.iter() {
        let mut count = 0;
        for card2 in cards.iter() {
            if card == card2 {
                count += 1;
            }
        }
        cards_count.push(count);
    }
    cards_count.sort();
    cards_count
}
