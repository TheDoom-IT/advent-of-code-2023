use std::fs;

pub fn main() {
    let content = fs::read_to_string("./src/day1/input.txt").expect("Unable to open a file.");

    let mut sum: u32 = 0;
    for line in content.lines() {
        let line = line
            .replace("one", "o1ne")
            .replace("two", "t2wo")
            .replace("three", "thr3ee")
            .replace("four", "fo4ur")
            .replace("five", "fi5ve")
            .replace("six", "s6ix")
            .replace("seven", "sev7en")
            .replace("eight", "eig8ht")
            .replace("nine", "ni9ne");

        let digits: Vec<u32> = line
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        let first_digit = digits.first().expect("Every row should have a digit");
        let last_digit = digits.last().expect("Every row should have a digit");
        let calibration_value = first_digit * 10 + last_digit;
        println!("{}: {}", line, calibration_value);
        sum += calibration_value;
    }

    println!("Sum: {}", sum);
}

// first part: 56506
// second part: 56017
