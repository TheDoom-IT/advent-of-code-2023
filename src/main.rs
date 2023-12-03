use std::env;

mod day1;
mod day2;
mod day3;

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
        _ => println!("Day {} not implemented yet", day_number),
    }
}
