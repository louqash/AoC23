fn is_character(c: char) -> bool {
    !c.is_numeric() && c != '.'
}

pub fn solution(input: &String) -> i32 {
    let mut schematic: Vec<Vec<char>> = Vec::new();

    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        schematic.push(line.chars().collect());
    }

    let h = schematic.len();
    let w = schematic[0].len();
    let mut numbers: Vec<i32> = Vec::new();
    let mut prev_row: Option<&Vec<char>> = None;
    let mut next_row: Option<&Vec<char>>;
    for y in 0..h {
        let row = &schematic[y];
        next_row = if y == h-1 {
            None
        } else {
            Some(&schematic[y+1])
        };

        let mut number: i32 = 0;
        let mut valid = false;
        for x in 0..w {
            let c = row[x];
            if c.is_numeric() {
                number = number*10 + c.to_digit(10).unwrap() as i32;
                if !valid {
                    if let Some(pr) = prev_row {
                        if (x > 0 && is_character(pr[x-1])) || is_character(pr[x]) || (x < h-1 && is_character(pr[x+1])) {
                            valid = true;
                        } 
                    }
                    if let Some(nr) = next_row {
                        if (x > 0 && is_character(nr[x-1])) || is_character(nr[x]) || (x < h-1 && is_character(nr[x+1])) {
                            valid = true;
                        } 
                    }
                    if (x > 0 && is_character(row[x-1])) || (x < h-1 && is_character(row[x+1])) {
                        valid = true;
                    }
                }
            } else {
                if valid {
                    numbers.push(number);
                } else if number != 0{
                }
                number = 0;
                valid = false;
            }
        }
        if valid {
            numbers.push(number);
        }
        prev_row = Some(row);
    }
    
    numbers.iter().sum()
}
