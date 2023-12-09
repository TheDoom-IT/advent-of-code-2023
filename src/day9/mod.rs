use std::fs;

#[derive(Debug)]
struct History {
    numbers: Vec<i32>,
}
#[derive(Debug)]
struct HistorySequences<'a> {
    history: &'a History,
    subsequences: Vec<Vec<i32>>,
}

pub fn main() {
    let content = fs::read_to_string("./src/day9/input.txt").expect("Could not read file");

    let mut histories: Vec<History> = Vec::new();
    for line in content.lines() {
        let line_split = line.split(' ').collect::<Vec<&str>>();
        let numbers: Vec<i32> = line_split
            .iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        histories.push(History { numbers })
    }

    let expanded_histories: Vec<HistorySequences> =
        histories.iter().map(|h| expand_history(h)).collect();

    let next_elements: Vec<i32> = expanded_histories
        .iter()
        .map(|h| find_next_history_element(h))
        .collect();

    let result1 = next_elements.iter().sum::<i32>();
    println!("Task 1: {}", result1); // 1861775706

    let previous_elements: Vec<i32> = expanded_histories
        .iter()
        .map(|h| find_previous_history_element(h))
        .collect();

    let result2 = previous_elements.iter().sum::<i32>();
    println!("Task 2: {}", result2); // 1082
}

fn expand_history(history: &History) -> HistorySequences {
    let mut subsequences = Vec::new();

    let mut current_sequence = &history.numbers;
    loop {
        if current_sequence.iter().all(|n| *n == 0) {
            break;
        }

        let mut new_sequence = Vec::new();
        for i in 0..(current_sequence.len() - 1) {
            let new_element = current_sequence[i + 1] - current_sequence[i];
            new_sequence.push(new_element);
        }

        subsequences.push(new_sequence);
        current_sequence = subsequences.last().unwrap();
    }

    HistorySequences {
        history,
        subsequences,
    }
}

fn find_next_history_element(his_seq: &HistorySequences) -> i32 {
    let mut previous_last_element = 0;

    // we can ignore last sequence, because it's all zeros
    for i in (0..=(his_seq.subsequences.len() - 2)).rev() {
        let new_element = his_seq.subsequences[i].last().unwrap() + previous_last_element;
        previous_last_element = new_element;
    }

    previous_last_element + his_seq.history.numbers.last().unwrap()
}

fn find_previous_history_element(his_seq: &HistorySequences) -> i32 {
    let mut previous_first_element = 0;

    // we can ignore last sequence, because it's all zeros
    for i in (0..=(his_seq.subsequences.len() - 2)).rev() {
        let new_element = his_seq.subsequences[i].first().unwrap() - previous_first_element;
        previous_first_element = new_element;
    }

    his_seq.history.numbers.first().unwrap() - previous_first_element
}
