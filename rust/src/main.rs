use std::fs;
use std::io::{stdin, Write};

mod day1;
mod day2;
mod day3;
mod day4;

const CARGO_PROJECT_PATH: &str = env!("CARGO_MANIFEST_DIR");

fn get_input(day: i32) -> String {
    fs::read_to_string(format!("{}/inputs/day{}.txt", CARGO_PROJECT_PATH, day)).unwrap()
}

fn main() {

    let day_number;
    loop {
        print!("Which Day's solution do you want to get: ");
        let _ = std::io::stdout().flush();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        if let Ok(n) = line.trim().parse::<i32>() {
            println!("");
            day_number = n;
            break;
        } else {
            println!("You have to input a number.");
        }
    }

    match day_number {
        1 => {
            let input = get_input(1);
            println!("{}", "### DAY 1");
            println!("Part One Solution = {}", day1::part1::solution(&input));
            println!("Part Two Solution = {}", day1::part2::solution(&input));
        },
        2 => {
            let input = get_input(2);
            println!("{}", "### DAY 2");
            println!("Part One Solution = {}", day2::part1::solution(&input));
            println!("Part Two Solution = {}", day2::part2::solution(&input));
        },
        3 => {
            let input = get_input(3);
            println!("{}", "### DAY 3");
            println!("Part One Solution = {}", day3::part1::solution(&input));
            println!("Part Two Solution = {}", day3::part2::solution(&input));
        },
        4 => {
            let input = get_input(4);
            println!("{}", "### DAY 4");
            println!("Part One Solution = {}", day4::part1::solution(&input));
            println!("Part Two Solution = {}", day4::part2::solution(&input));
        },
        _ => {
            println!("The day {} is not implemented yet", day_number);
        },
    }

}
