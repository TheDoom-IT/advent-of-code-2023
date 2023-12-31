use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a day number");
    }
    let day_number = args[1]
        .parse::<u32>()
        .expect("Please provide a valid day number");

    println!("Day {}", day_number);
    match day_number {
        1 => day1::main(),
        2 => day2::main(),
        3 => day3::main(),
        4 => day4::main(),
        5 => day5::main(),
        6 => day6::main(),
        7 => day7::main(),
        8 => day8::main(),
        9 => day9::main(),
        _ => println!("Day {} not implemented yet", day_number),
    }
}
