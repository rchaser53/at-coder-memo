#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input! {
    n: usize,
    s: Chars
  }

  let mut count = 0;
  let mut max = 0;
  for c in s {
    if c == 'I' {
      count += 1;
    } else {
      count -= 1;
    }
    max = std::cmp::max(max, count);
  }
  println!("{}", max);
}