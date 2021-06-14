/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::{input};
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 1_000_000_007;

pub fn main(
) {
  input! {
    n:usize
  }

  let mut result = 0usize;
  for i in 1..=n {
    if i % 3 != 0 && i % 5 != 0 {
      result += i;
    }
  }
  
  println!("{}", result);
}