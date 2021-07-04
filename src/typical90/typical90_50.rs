#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

const MOD:usize = 1_000_000_007;
fn main() {
  input! {
    n:usize,
    k:usize,
  }

  let mut memo = vec![0;n+1];
  memo[0] = 1;

  for i in 1..=n {
    if k <= i {
      memo[i] += memo[i-k];
      memo[i] %= MOD;
    }
    memo[i] += memo[i-1];
    memo[i] %= MOD;
  }
  println!("{}", memo[n]);
}
