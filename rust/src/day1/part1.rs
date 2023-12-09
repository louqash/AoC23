pub fn solution(input: &String) -> i32 {
  let mut sum: i32 = 0;
  for line in input.split("\n") {
    let mut first = None;
    let mut last = None;
    for c in line.chars() {
      if let Some(n) = c.to_digit(10) {
        if first.is_none() {
          first = Some(n);
        }
        last = Some(n);
      }
    }
    if let Some(first) = first {
      sum += (first * 10 + last.unwrap()) as i32;
    }
  }
  sum
}

