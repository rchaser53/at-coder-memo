use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    s:Chars
  }

  let mut ai = s.len();
  let mut zi = 0;
  for i in 0..s.len() {
    if s[i] == 'A' {
      ai = std::cmp::min(i, ai);
    }

    if s[i] == 'Z' {
      zi = std::cmp::max(zi, i);
    }
  }
  println!("{}", zi - ai + 1);
}