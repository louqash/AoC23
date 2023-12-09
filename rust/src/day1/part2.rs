fn find_first_and_last(input: &String, pattern: &str) -> Option<(usize, usize)>{
    let mut matcher = input.match_indices(pattern);
    let first_match = matcher.next();
    if first_match.is_none() {
      return None
    }
    let first = first_match.map(|tuple| tuple.0).unwrap();
    let last = matcher.last().map_or(first, |tuple| tuple.0);
    Some((first, last))
}

pub fn solution(input: &String) -> i32 {
  let mut sum: i32 = 0;
  for line in input.split("\n") {
    let line = line.to_string();
    let word_positions = [
      find_first_and_last(&line, "one"),
      find_first_and_last(&line, "two"),
      find_first_and_last(&line, "three"),
      find_first_and_last(&line, "four"),
      find_first_and_last(&line, "five"),
      find_first_and_last(&line, "six"),
      find_first_and_last(&line, "seven"),
      find_first_and_last(&line, "eight"),
      find_first_and_last(&line, "nine"),
    ];

    // Tuple (position of number word, the corresponding numerical value)
    let mut first = (usize::MAX, 0);
    let mut last = (0, 0);
    for (number, word_position) in word_positions.into_iter().enumerate() {
      if let Some((first_position, last_position)) = word_position {
        let number = number as u32;
        if first.0 > first_position {
          first.0 = first_position;
          first.1 = number + 1;
        }
        if last.0 < last_position {
          last.0 = last_position;
          last.1 = number + 1;
        }
      }
    }

    for (index, c) in line.chars().enumerate() {
      if let Some(n) = c.to_digit(10) {
        if first.0 > index {
          first.0 = index;
          first.1 = n;
        }
        if last.0 <= index {
          last.0 = index;
          last.1 = n;
        }
      }
    }
    sum += (first.1 * 10 + last.1) as i32;
  }
  sum
}

