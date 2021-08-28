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
    mut vals:[(usize,usize);n]
  }
  vals.reverse();
  let mut result = 0;
  
  for (a, b) in vals {
    let nv = (a + result) % b;
    if nv != 0 {
      result += b - nv;
    }
  }
  println!("{}", result);
}
