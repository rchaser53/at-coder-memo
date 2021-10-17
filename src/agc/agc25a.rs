use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
  }

  let mut min = 1_000_000_000usize;
  for a in 1..n {
    let b = n - a;
    let mut temp = 0;
    for c in a.to_string().chars() {
      temp += (c as u8 - '0' as u8) as usize;
    }
    for c in b.to_string().chars() {
      temp += (c as u8 - '0' as u8) as usize;
    }
    min = std::cmp::min(min, temp);
  }
  println!("{}", min);
}