#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 1_000_000_007;
fn main() {
  input! {
    n:usize,
    mut vals:[usize;n]
  }
  vals.insert(0, 0);
  vals.sort();
  
  let mut result = 1;
  for i in 0..n {
    result *= (vals[i+1] - vals[i] + 1);
    result %= MOD;
  }
  println!("{}", result);
}