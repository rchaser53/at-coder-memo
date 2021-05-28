/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

const MOD:usize = 1_000_000_007;
pub fn main(
) {
  input! {
    n:usize,
    vals:[[usize;6];n]
  }
  let mut result = 1;
  for i in 0..n {
    result *= vals[i].iter().sum::<usize>();
    result %= MOD;
  }
  
  println!("{}", result);
}
