use proconio::input;
use std::collections::HashMap;

fn main() {
  input! {
    N: i128,
    K: i128,
  }
  
  let quotient = N / K;
  let mut n = if 0 < quotient {
  	N - (K * quotient)
  } else {
    N
  };
    
  let mut map: HashMap<i128, bool> = HashMap::new();
  let mut result = i128::max_value();
  while map.get(&n).is_none() {
    map.insert(n, true);
    result = std::cmp::min(n, result);
    n = (n - K).abs();
  }
  println!("{}", result);
}