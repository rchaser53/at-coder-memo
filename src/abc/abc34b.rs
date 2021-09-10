use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    n:usize
  }

  if n % 2 == 0 {
    println!("{}", n - 1);
  } else {
    println!("{}", n + 1);
  }
}