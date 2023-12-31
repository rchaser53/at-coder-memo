/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

const MOD:usize = 998244353;
fn main() {
  input! {
    n:usize,
    m:usize
  }

  let mut same = m;
  let mut diff = 0;
  for _ in 0..n-1 {
    let new_same = diff;
    let mut new_diff = same * (m-1) % MOD;
    new_diff += diff * (m-2) % MOD;
    new_diff %= MOD;

    same = new_same;
    diff = new_diff;
  }

  println!("{}", diff);
}