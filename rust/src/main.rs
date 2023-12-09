use std::fs;

mod day1;

const CARGO_PROJECT_PATH: &str = env!("CARGO_MANIFEST_DIR");

fn get_input(day: i32) -> String {
    fs::read_to_string(format!("{}/inputs/day{}.txt", CARGO_PROJECT_PATH, day)).unwrap()
}

fn main() {
    let input = get_input(1);
    println!("{}", "DAY 1");
    println!("Part One Solution = {}", day1::part1::solution(&day1_input));
    println!("Part Two Solution = {}", day1::part2::solution(&day1_input));
}
