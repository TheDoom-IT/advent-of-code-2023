use std::fs;

#[derive(Debug)]
struct Round {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

pub fn main() {
    let content =
        fs::read_to_string("src/day2/input.txt").expect("Something went wrong reading the file");
    let mut games: Vec<Game> = Vec::new();

    // parsing data...
    for line in content.lines() {
        let line_split: Vec<&str> = line.split(":").collect();
        let game_str = line_split[0].trim();
        let game_split: Vec<&str> = game_str.split(" ").collect();
        let game_id: u32 = game_split[1].trim().parse().expect("Not a number");

        let mut rounds: Vec<Round> = Vec::new();
        let rounds_str = line_split[1].trim();
        let rounds_split: Vec<&str> = rounds_str.split(";").collect();

        for round_str in rounds_split {
            let mut round = Round {
                red: None,
                green: None,
                blue: None,
            };
            let round_split: Vec<&str> = round_str.trim().split(",").collect();
            for round_item in round_split {
                let round_item_split: Vec<&str> = round_item.trim().split(" ").collect();
                let round_item_number: u32 =
                    round_item_split[0].trim().parse().expect("Not a number");
                let round_item_color = round_item_split[1].trim();
                match round_item_color {
                    "red" => round.red = Some(round_item_number),
                    "green" => round.green = Some(round_item_number),
                    _ => round.blue = Some(round_item_number),
                }
            }
            rounds.push(round);
        }
        games.push(Game {
            id: game_id,
            rounds,
        })
    }

    let max_red: u32 = 12;
    let max_green: u32 = 13;
    let max_blue: u32 = 14;

    // task 1
    let possible_games: Vec<&Game> = games
        .iter()
        .filter(|game| {
            let mut is_possible = true;
            for round in &game.rounds {
                if let Some(red) = round.red {
                    if red > max_red {
                        is_possible = false;
                        break;
                    }
                }
                if let Some(green) = round.green {
                    if green > max_green {
                        is_possible = false;
                        break;
                    }
                }
                if let Some(blue) = round.blue {
                    if blue > max_blue {
                        is_possible = false;
                        break;
                    }
                }
            }
            is_possible
        })
        .collect();

    let sum = possible_games.iter().map(|game| game.id).sum::<u32>();
    println!("Task 1: {}", sum); // 2207

    // task 2
    let mut max_rounds: Vec<Round> = Vec::new();
    for game in games {
        let mut max_round = Round {
            red: Some(0),
            green: Some(0),
            blue: Some(0),
        };

        for round in game.rounds {
            if let Some(red) = round.red {
                if let Some(round_red) = max_round.red {
                    if red > round_red {
                        max_round.red = Some(red);
                    }
                }
            }
            if let Some(green) = round.green {
                if let Some(round_green) = max_round.green {
                    if green > round_green {
                        max_round.green = Some(green);
                    }
                }
            }
            if let Some(blue) = round.blue {
                if let Some(round_blue) = max_round.blue {
                    if blue > round_blue {
                        max_round.blue = Some(blue);
                    }
                }
            }
        }
        max_rounds.push(max_round);
    }

    let powers: u32 = max_rounds
        .iter()
        .map(|round| {
            let red = round.red.unwrap();
            let green = round.green.unwrap();
            let blue = round.blue.unwrap();
            red * green * blue
        })
        .sum();

    println!("Task 2: {}", powers); // 62241
}
