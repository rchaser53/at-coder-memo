#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input! {
    n: usize,
    a: usize,
    b: usize,
    vals: [usize;n]
  }

  let mut count = 0;
  let mut last = vals[0];
  for i in 1..n {
    let current = vals[i];
    let walk = (current - last) * a;
    if walk < b {
      count += walk;
    } else {
      count += b;
    }
    last = current;
  }
  println!("{}", count);
}