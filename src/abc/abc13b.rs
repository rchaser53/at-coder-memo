use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    a:isize,
    b:isize,
  }

  let mut result = 10isize;
  for i in 0..=10 {
    if (b + i) % 10 == a {
      result = std::cmp::min(result, i);
    }
    if (10 + b - i).abs() % 10 == a {
      result = std::cmp::min(result, i);
    }
  }
  println!("{}", result);
  
}