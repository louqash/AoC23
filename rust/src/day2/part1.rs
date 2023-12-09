pub fn solution(input: &String) -> i32 {
    let mut sum: i32 = 0;

    const RED_LIMIT: i32 = 12;
    const GREEN_LIMIT: i32 = 13;
    const BLUE_LIMIT: i32 = 14;

    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }

        let mut line_split = line.split(':');
        let game = line_split.next().unwrap();
        let game_id: i32 = game.split(' ').last().unwrap().parse().unwrap();

        let mut possible = true;
        let cubes = line_split.next().unwrap();
        'outer: for draw in cubes.split(';') {
            for cube_set in draw.split(',') {
                let mut iterator = cube_set.trim().split(' ');
                let num: i32 = iterator.next().unwrap().parse().unwrap();
                let color = iterator.next().unwrap();

                match color {
                    "red" => {
                        if num > RED_LIMIT {
                            possible = false;
                            break 'outer;
                        }
                    },
                    "green" => {
                        if num > GREEN_LIMIT {
                            possible = false;
                            break 'outer;
                        }
                    },
                    "blue" => {
                        if num > BLUE_LIMIT {
                            possible = false;
                            break 'outer;
                        }
                    },
                    _ => {}
                }
            }
        }
        if possible {
            sum += game_id;
        }
    }
    sum
}

