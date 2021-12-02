/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

const MOD:usize = 1_000_000_007;
fn main() {
  input! {
    n:usize,
    mut vals:[usize;n]
  }

  vals.sort();
  vals.reverse();

  let mut result = 0usize;
  for i in 0..n {
    result += (i+2) % MOD * vals[i] % MOD;
  }
  for _ in 0..n-1 {
    result *= 4;
    result %= MOD;
  }
  println!("{}", result);
}