pub fn solution(input: &String) -> i32 {
    let mut sum: i32 = 0;

    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }

        let line_split = line.split(':');
        //let game = line_split.next().unwrap();
        //let game_id: i32 = game.split(' ').last().unwrap().parse().unwrap();

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        let cubes = line_split.last().unwrap();
        for draw in cubes.split(';') {
            for cube_set in draw.split(',') {
                let mut iterator = cube_set.trim().split(' ');
                let num: i32 = iterator.next().unwrap().parse().unwrap();
                let color = iterator.next().unwrap();

                match color {
                    "red" => {
                        if num > max_red {
                            max_red = num;
                        }
                    },
                    "green" => {
                        if num > max_green {
                            max_green = num;
                        }
                    },
                    "blue" => {
                        if num > max_blue {
                            max_blue = num;
                        }
                    },
                    _ => {}
                }
            }
        }
        sum += max_red * max_green * max_blue;
    }
    sum
}

