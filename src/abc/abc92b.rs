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
    d:usize,
    x:usize,
    vals:[usize;n] 
  }
  let mut result = x;
  for v in vals {
    let mut i = 0;
    while i * v < d {
      result += 1;
      i += 1;
    }
    // println!("{}", result);
  }
  println!("{}", result);
}
