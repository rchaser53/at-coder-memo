use proconio::input;
use proconio::marker::*;

fn main() {
  input! {
    n: usize,
    chars: Chars
  }
  
  let mut min = usize::max_value();
  let mut memo = vec![(0,0);n];
  
  let mut w = 0;
  for i in 0..n-1 {
    if chars[i] == 'W' {
      w += 1;
    }
    memo[i+1].0 = w;
  }
  
  let mut e = 0;
  for i in (0..n-1).rev() {
    if chars[i+1] == 'E' {
      e += 1;
    }
    memo[i].1 = e;
  }
  
  let mut min = usize::max_value();
  for (w, e) in memo {
    min = std::cmp::min(min, w + e);
  }
  println!("{}", min);
}