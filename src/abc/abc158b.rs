/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::{input};
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    mut n:usize,
    a:usize,
    b:usize
  }
  let total = a + b;

  let c = n / total;
  n -= c * total;
  if n <= a {
    println!("{}", c * a + n);
  } else {
    println!("{}", c * a + a);
  }
}