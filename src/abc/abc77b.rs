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
  }
  let mut i = 1;
  while i * i <= n {
    i += 1;
  }
  println!("{}", (i - 1).pow(2));
}
