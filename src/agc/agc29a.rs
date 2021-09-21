use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    s:Chars
  }

  let mut count = 0usize;
  let mut ws = 0usize;
  for i in (0..s.len()).rev() {
    if s[i] == 'W' {
      ws += 1;
    } else {
      count += ws;
    }
  }
  println!("{}", count);
}
