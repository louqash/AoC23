pub fn solution(input: &String) -> i32 {
    let mut sum: i32 = 0;

    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }

        let mut line_split = line.split(':');
        let game = line_split.next().unwrap();
        let _game_id: i32 = game.split(' ').last().unwrap().parse().unwrap();

        let numbers = line_split.next().unwrap();
        let mut numbers_split = numbers.split('|');
        let winning_numbers_str = numbers_split.next().unwrap().trim();
        let my_numbers_str = numbers_split.next().unwrap().trim();


        let mut winning_numbers = Vec::new();
        for number in winning_numbers_str.split(' ') {
            if number.is_empty() {
                continue;
            }
            winning_numbers.push(number.trim().parse::<u32>().unwrap());
        }

        let mut points = 0;
        for number in my_numbers_str.split(' ') {
            if number.is_empty() {
                continue;
            }
            if winning_numbers.contains(&number.trim().parse::<u32>().unwrap()) {
                if points > 0 {
                    points *= 2;
                } else {
                    points = 1;
                }
            }
        }
        sum += points;
    }
    sum
}

