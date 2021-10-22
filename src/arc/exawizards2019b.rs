use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input!{
    n:usize,
    s:Chars
  }

  let mut r = 0;
  let mut b = 0;
  for c in s {
    if c == 'R' {
      r += 1;
    } else {
      b += 1;
    }
  }

  if b < r {
    println!("Yes");
  } else {
    println!("No");
  }
}