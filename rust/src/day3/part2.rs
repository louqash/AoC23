use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point(usize, usize);

type AsteriskToNumbersMap = HashMap<Point, ([Option<u32>; 2], bool)>;

fn add_number_neighbouring_with_asterisks(number: u32, asterisks: &Vec<Point>, asterisk_to_numbers: &mut AsteriskToNumbersMap) {
    for asterisk_position in asterisks {
        if !asterisk_to_numbers.contains_key(asterisk_position) {
            asterisk_to_numbers.insert(*asterisk_position, ([None, None], true));
        }
        let neightbouring_numbers = asterisk_to_numbers.get_mut(asterisk_position).unwrap();
        if neightbouring_numbers.1 == false {
            continue;
        }
        if neightbouring_numbers.0[0].is_some() && neightbouring_numbers.0[1].is_some() {
            neightbouring_numbers.1 = false;
            continue;
        }
        if neightbouring_numbers.0[0].is_some() {
            neightbouring_numbers.0[1] = Some(number);
        } else {
            neightbouring_numbers.0[0] = Some(number);
        }
    }
}

pub fn solution(input: &String) -> u32 {
    let mut schematic: Vec<Vec<char>> = Vec::new();

    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        schematic.push(line.chars().collect());
    }

    let h = schematic.len();
    let w = schematic[0].len();
    let mut asterisk_to_numbers: AsteriskToNumbersMap = HashMap::new();
    let mut prev_row: Option<&Vec<char>> = None;
    let mut next_row: Option<&Vec<char>>;
    for y in 0..h {
        let row = &schematic[y];
        next_row = if y == h-1 {
            None
        } else {
            Some(&schematic[y+1])
        };

        let mut number: u32 = 0;
        let mut valid = false;
        let mut asterisks = Vec::new();
        for x in 0..w {
            let c = row[x];
            if c.is_numeric() {
                number = number*10 + c.to_digit(10).unwrap();
                if let Some(pr) = prev_row {
                    if x > 0 && !row[x-1].is_numeric() && pr[x-1] == '*' {
                        valid = true;
                        asterisks.push(Point(x-1, y-1));
                    }
                    if pr[x] == '*' {
                        valid = true;
                        asterisks.push(Point(x, y-1));
                    }
                    if x < h-1 && !row[x+1].is_numeric() && pr[x+1] == '*' {
                        valid = true;
                        asterisks.push(Point(x+1, y-1));
                    }
                }
                if let Some(nr) = next_row {
                    if x > 0 && !row[x-1].is_numeric() && nr[x-1] == '*' {
                        valid = true;
                        asterisks.push(Point(x-1, y+1));
                    }
                    if nr[x] == '*' {
                        valid = true;
                        asterisks.push(Point(x, y+1));
                    }
                    if x < h-1 && !row[x+1].is_numeric() && nr[x+1] == '*' {
                        valid = true;
                        asterisks.push(Point(x+1, y+1));
                    }
                }
                if x > 0 && row[x-1] == '*' {
                    valid = true;
                    asterisks.push(Point(x-1, y));
                }
                if x < h-1 && row[x+1] == '*' {
                    valid = true;
                    asterisks.push(Point(x+1, y));
                }
            } else {
                if valid {
                    add_number_neighbouring_with_asterisks(number, &asterisks, &mut asterisk_to_numbers)
                }
                number = 0;
                valid = false;
                asterisks.clear();
            }
        }
        if valid {
            add_number_neighbouring_with_asterisks(number, &asterisks, &mut asterisk_to_numbers)
        }
        prev_row = Some(row);
    }
    
    let mut sum = 0;
    for (_, (numbers, is_ok)) in &asterisk_to_numbers {
        if *is_ok && numbers[0].is_some() && numbers[1].is_some() {
            sum += numbers[0].unwrap() * numbers[1].unwrap();
        }
    }
    sum
}
