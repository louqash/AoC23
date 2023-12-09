use std::fs;

mod day1;
mod day2;
mod day3;

const CARGO_PROJECT_PATH: &str = env!("CARGO_MANIFEST_DIR");

fn get_input(day: i32) -> String {
    fs::read_to_string(format!("{}/inputs/day{}.txt", CARGO_PROJECT_PATH, day)).unwrap()
}

fn main() {
    let input = get_input(1);
    println!("{}", "DAY 1");
    println!("Part One Solution = {}", day1::part1::solution(&input));
    println!("Part Two Solution = {}", day1::part2::solution(&input));

    let input = get_input(2);
    println!("{}", "DAY 2");
    println!("Part One Solution = {}", day2::part1::solution(&input));
    println!("Part Two Solution = {}", day2::part2::solution(&input));

    let input = get_input(3);
    println!("{}", "DAY 3");
    println!("Part One Solution = {}", day3::part1::solution(&input));
    println!("Part Two Solution = {}", day3::part2::solution(&input));
}
