/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

pub fn main(
) {
  input! {
    n:usize,
    t:usize,
    vals:[(usize,usize);n]
  }

  let mut min = 1_000_000_000;
  for (c, v) in vals {
    if v <= t {
      min = std::cmp::min(c, min);
    }
  }
  if min == 1_000_000_000 {
    println!("TLE");
  } else {
    println!("{}", min);
  }
}