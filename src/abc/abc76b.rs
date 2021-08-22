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
    k:usize,
  }
  
  let mut min = 1_000_000_000;
  let limit = 1 << n;
  for i in 0..limit {
    let mut temp = 1;
    for j in 0..n {
      if i >> j & 1 == 1 {
        temp *= 2;
      } else {
        temp += k;
      }
    }
    min = std::cmp::min(min, temp);
  }
  println!("{}", min);
}
