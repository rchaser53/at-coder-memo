use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    n:usize,
    vals:[isize;n]
  }

  let mut memo = vec![0;n+1];
  memo[0] = vals[0];
  for i in 0..n {
    if i % 2 == 0 {
      memo[i+1] = memo[i] + vals[i];
    } else {
      memo[i+1] = memo[i] - vals[i];
    }
  }
  let mut map = HashMap::new();
  let mut result = 0usize;
  for v in memo {
    let entry = map.entry(v).or_insert(0usize);
    result += *entry;
    *entry += 1;
  }

  println!("{}", result);
}