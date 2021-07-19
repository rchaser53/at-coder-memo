#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    r:usize,
    d:usize,
    mut val:usize
  }

  for _ in 0..10 {
    val *= r;
    val -= d;
    println!("{}", val);
  }
}
