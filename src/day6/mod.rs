use std::fs;

struct Race {
    time: u64,
    distance: u64,
}

pub fn main() {
    let content = fs::read_to_string("./src/day6/input.txt").expect("Could not read file");

    let lines: Vec<&str> = content.lines().collect();
    if lines.len() != 2 {
        panic!("Invalid input");
    }

    let times = get_numbers_from_line(lines[0]);
    let distances = get_numbers_from_line(lines[1]);

    let races: Vec<Race> = times
        .iter()
        .enumerate()
        .map(|(index, time)| Race {
            time: *time,
            distance: distances[index],
        })
        .collect();

    // task 1
    let mut races_solutions: Vec<u64> = Vec::new();

    for race in races {
        races_solutions.push(find_solutions_number_for_race(&race));
    }

    let result1 = races_solutions.iter().product::<u64>();
    println!("Task 1: {}", result1); //252000

    //task 2
    let race = Race {
        time: get_number_from_line(lines[0]),
        distance: get_number_from_line(lines[1]),
    };

    let result2 = find_solutions_number_for_race(&race);
    println!("Task 2: {}", result2); //36992486
}

fn find_solutions_number_for_race(race: &Race) -> u64 {
    let mut solutions_number = 0;
    for i in 1..race.time {
        let velocity = i;
        let distance = (race.time - i) * velocity;
        if distance > race.distance {
            solutions_number += 1;
        }
    }
    solutions_number
}

fn get_numbers_from_line(line: &str) -> Vec<u64> {
    let line_split = line.split(':').collect::<Vec<&str>>();
    if line_split.len() != 2 {
        panic!("Invalid input");
    }

    line_split[1]
        .trim()
        .split(' ')
        .map(|x| x.parse::<u64>())
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect()
}

fn get_number_from_line(line: &str) -> u64 {
    let line_split = line.split(':').collect::<Vec<&str>>();
    if line_split.len() != 2 {
        panic!("Invalid input");
    }

    let number = line_split[1].replace(" ", "").parse::<u64>();
    match number {
        Ok(number) => number,
        Err(e) => {
            panic!("Invalid input")
        }
    }
}
