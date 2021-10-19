use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    n:usize,
    mut vals:[usize;n]
  }

  let mut count = 0;
  let base = 1_000_000;
  for i in 1..n {
    if vals[i-1] == vals[i] {
      vals[i] = base + count;
      count += 1;
    }
  }  
  println!("{}", count);
}