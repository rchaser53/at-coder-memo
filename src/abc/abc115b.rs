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
    mut vals:[usize;n]
  }
  vals.sort();
  let mut result = 0;
  for i in 0..n-1 {
    result += vals[i];
  }

  println!("{}", result+vals[n-1]/2);
}
