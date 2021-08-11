/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    m:usize,
    x:Usize1,
    vals:[Usize1;m]
  }

  let mut memo = vec![0;n];
  for i in vals {
    memo[i] = 1;
  }

  let mut left = 0;
  let mut right = 0;
  for i in 0..x {
    left += memo[i];
  }

  for i in x+1..n {
    right += memo[i];
  }
  println!("{}", std::cmp::min(left, right));
}
