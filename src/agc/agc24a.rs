use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input!{
    a:isize,
    b:isize,
    c:isize,
    k:usize
  }

  let diff = a - b;
  if k % 2 == 0 {
    println!("{}", diff);
  } else {
    println!("{}", -1 * diff);
  }
}