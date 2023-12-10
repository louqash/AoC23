pub fn solution(input: &String) -> i32 {
    let num_games = input.as_bytes().iter().filter(|&&c| c == b'\n').count();
    let mut number_of_won_scratchcards: Vec<i32> = vec![1; num_games];

    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }

        let mut line_split = line.split(':');
        let game = line_split.next().unwrap();
        let game_id: usize = game.split(' ').last().unwrap().parse().unwrap();

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

        let mut matching_numbers = 0;
        for number in my_numbers_str.split(' ') {
            if number.is_empty() {
                continue;
            }
            if winning_numbers.contains(&number.trim().parse::<u32>().unwrap()) {
                matching_numbers += 1;
            }
        }
        for i in 0..matching_numbers {
            let idx = game_id + i;
            if idx > num_games {
                break;
            }
            number_of_won_scratchcards[game_id + i] += number_of_won_scratchcards[game_id-1];
        }
    }

    number_of_won_scratchcards.iter().sum()
}

