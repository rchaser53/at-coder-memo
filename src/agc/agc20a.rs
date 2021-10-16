use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:isize,
    b:isize
  }

  let diff = (a-b).abs();
  if diff % 2 == 0 {
    println!("Alice");
  } else {
    println!("Borys");
  }
}