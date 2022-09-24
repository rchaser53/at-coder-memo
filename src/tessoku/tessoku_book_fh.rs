/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;

const MOD:usize = 1_000_000_007;
fn repeat_square(n:usize, p:usize) -> usize {
  if p == 0 {
    1
  } else if p % 2 == 0 {
    repeat_square(n, p/2).pow(2) % MOD 
  } else {
    n * repeat_square(n, p-1) % MOD
  }
}

fn main() { 
  input! { 
    w:usize,
  }

  println!("{}", 12 * repeat_square(7, w-1) % MOD);
}