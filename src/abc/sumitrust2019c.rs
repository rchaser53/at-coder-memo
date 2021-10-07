use proconio::input;
use proconio::marker::*;
use std::collections::*;
 
pub fn main(
) {
  input! {
    x:usize,
  }

  let mut memo = vec![1000;100];
  memo[0] = 0;

  for i in 0..100 {
    for j in 1..=5 {
      if 100 <= i + j { continue }
      memo[i+j] = std::cmp::min(memo[i+j], memo[i] + 1);
    }
  }

  let v = x / 100;
  let av = x % 100;
  if v < memo[av] {
    println!("0");
  } else {
    println!("1");
  }
}