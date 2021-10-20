use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    s:Chars
  }

  let mut result = s.len();
  for i in 1..s.len() {
    if s[i-1] != s[i] {
      let next = std::cmp::max(i, s.len()-i);
      result = std::cmp::min(next, result);
    }
  }
  println!("{}", result);
}