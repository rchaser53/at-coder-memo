/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 1_000_000_007;
pub fn main(
) {
  input! {
    n:usize,
    vals:[usize;n]
  }
  
  let mut total = 0;
  for i in 0..n {
    for j in i+1..n {
      total += vals[i] * vals[j];
    }
  }
  println!("{}", total);
}
