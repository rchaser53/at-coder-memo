/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

pub fn main(
) {
  input! {
    n:usize,
    vals:[usize;n]
  }

  let mut maxes = vec![vals[0];n];
  for i in 1..n {
    maxes[i] = std::cmp::max(maxes[i-1], vals[i]);
  }

  let mut culculative = 0;
  let mut base = 0;
  for i in 0..n {
    let v = vals[i];
    println!("{}", base + culculative + v + maxes[i] * (i+1));
    culculative += v;
    base += culculative;
  }
}
