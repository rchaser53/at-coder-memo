use proconio::input;

fn main() {
  input! {
    _n: usize,
    k: usize,
    s: String,
  }
  let s = s.chars().collect::<Vec<char>>();
  let k = 2 * k + 1;
  let mut stack: Vec<usize> = vec![];
  
  if s[0] == '0' {
    stack.push(0);
  }
  
  let mut count = 0;
  let mut last = s[0];
  for c in s {
    if c == last {
      count += 1;
    } else {
      last = c;
      stack.push(count);
      count = 1;
    }
  }

  if count > 0 {
    stack.push(count);
  }

  let mut max = 0;
  let mut total = 0;
  let mut index = 0;
  let mut left = 0;
  let mut right = 0;
  while index < stack.len() {
    let next_left = index;
    let next_right = std::cmp::min(index + k, stack.len());

    while right < next_right {
      total += stack[right];
      right += 1;
    }
    
    while left < next_left {
      total -= stack[left];
      left += 1;
    }

    max = std::cmp::max(max, total);
    index += 2;
  }

  println!("{}", max);
}