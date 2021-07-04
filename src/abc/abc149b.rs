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
    mut a:usize,
    mut b:usize,
    mut k:usize
  }

  if k <= a {
    println!("{} {}", a-k, b);
    return
  }

  k -= a;
  a = 0;

  if k <= b {
    println!("{} {}", a, b-k);
  } else {
    println!("0 0");
  }
}