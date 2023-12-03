use std::fs;

fn is_adjacent_to_symbol(line: Option<&Vec<char>>, start: usize, end: usize) -> bool {
    let line = match line {
        Some(line) => line,
        None => return false,
    };

    let start = if start == 0 { 0 } else { start - 1 };
    for i in start..=(end + 1) {
        if let Some(c) = line.get(i) {
            if !c.is_digit(10) && *c != '.' {
                return true;
            }
        }
    }

    return false;
}

fn check_adjacent_symbols(
    lines: &Vec<Vec<char>>,
    line_index: usize,
    start: usize,
    end: usize,
) -> bool {
    let prev_line = if line_index == 0 {
        None
    } else {
        lines.get(line_index - 1)
    };
    for line in [prev_line, lines.get(line_index), lines.get(line_index + 1)] {
        let result = is_adjacent_to_symbol(line, start, end);
        if result {
            return true;
        }
    }

    return false;
}

fn handle_gear_number(line: &Vec<char>, digit_index: usize) -> u32 {
    let mut lindex: usize = digit_index;
    // go left
    for (index, char) in line.iter().enumerate().rev().skip(line.len() - digit_index) {
        if !char.is_digit(10) || index == 0 {
            lindex = index + 1;
            break;
        }
    }

    // go right
    let mut rindex: usize = digit_index;
    for (index, char) in line.iter().enumerate().skip(digit_index + 1) {
        if !char.is_digit(10) || index == line.len() - 1 {
            rindex = index - 1;
            break;
        }
    }

    let mut result: u32 = 0;
    for c in line.iter().skip(lindex).take(rindex - lindex + 1) {
        result = result * 10 + c.to_digit(10).unwrap();
    }
    result
}

fn look_for_number_at(line: Option<&Vec<char>>, index: usize) -> Option<u32> {
    if let Some(line) = line {
        if let Some(c) = line.get(index) {
            if c.is_digit(10) {
                return Some(handle_gear_number(line, index));
            }
        }
    }

    return None;
}
fn handle_gear(
    prev_line: Option<&Vec<char>>,
    curr_line: &Vec<char>,
    next_line: Option<&Vec<char>>,
    gear_index: usize,
) -> Option<u32> {
    let mut numbers: Vec<u32> = Vec::new();

    //curr line
    //left
    if gear_index > 0 {
        look_for_number_at(Some(curr_line), gear_index - 1).and_then(|number| {
            numbers.push(number);
            Some(())
        });
    }
    //right
    look_for_number_at(Some(curr_line), gear_index + 1).and_then(|number| {
        numbers.push(number);
        Some(())
    });

    //top, bottom
    for line in [prev_line, next_line] {
        look_for_number_at(line, gear_index)
            .and_then(|number| {
                numbers.push(number);
                Some(())
            })
            .or_else(|| {
                // corners
                if gear_index > 0 {
                    look_for_number_at(line, gear_index - 1).and_then(|number| {
                        numbers.push(number);
                        Some(())
                    });
                }
                look_for_number_at(line, gear_index + 1).and_then(|number| {
                    numbers.push(number);
                    Some(())
                })
            });
    }

    if numbers.len() == 2 {
        return Some(numbers.iter().product::<u32>());
    }
    return None;
}

pub fn main() {
    let content =
        fs::read_to_string("./src/day3/input.txt").expect("Something went wrong reading the file");

    let lines: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();

    let mut numbers: Vec<u32> = Vec::new();

    for (line_index, line) in lines.iter().enumerate() {
        let mut digit_start: Option<usize> = None;
        let mut current_number = 0;
        for (c_index, c) in line.iter().enumerate() {
            let is_current_digit = c.is_digit(10);
            if digit_start != None && is_current_digit {
                current_number = current_number * 10 + c.to_digit(10).unwrap();
            } else if digit_start != None && !is_current_digit {
                // handle digit
                let start = digit_start.unwrap();
                let end = c_index - 1;
                let result = check_adjacent_symbols(&lines, line_index, start, end);
                if result {
                    numbers.push(current_number);
                }
                digit_start = None;
                current_number = 0;
            } else if is_current_digit {
                digit_start = Some(c_index);
                current_number = c.to_digit(10).unwrap();
            }
        }
        if digit_start != None {
            let start = digit_start.unwrap();
            let end = line.len() - 1;
            let result = check_adjacent_symbols(&lines, line_index, start, end);
            if result == true {
                numbers.push(current_number);
            }
        }
    }

    let sum = numbers.iter().sum::<u32>();
    println!("Task 1: {}", sum); // 525911

    // Task 2
    let mut gear_ratios: u32 = 0;
    for (line_index, line) in lines.iter().enumerate() {
        for (c_index, c) in line.iter().enumerate() {
            if *c == '*' {
                let result = handle_gear(
                    if line_index == 0 {
                        None
                    } else {
                        lines.get(line_index - 1)
                    },
                    line,
                    lines.get(line_index + 1),
                    c_index,
                );
                if let Some(result) = result {
                    gear_ratios += result;
                }
            }
        }
    }

    println!("Task 2: {}", gear_ratios); // 75805607
}
