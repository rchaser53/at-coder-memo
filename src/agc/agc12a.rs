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
    mut vals:[usize;3*n]
  }

  vals.sort();
  vals.reverse();

  let mut result = 0;
  let mut i = 0;
  while i < 2 * n {
    result += vals[i+1];
    i += 2;
  }
  println!("{}", result);
}
