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
    mut x:usize
  }

  let mut result = 0usize;
  if x / 500 != 0 {
    let v = x / 500;
    result += 1000 * v;
    x %= 500;
  }
  if x / 5 != 0 {
    let v = x / 5;
    result += 5 * v;
  }
  
  println!("{}", result);
}