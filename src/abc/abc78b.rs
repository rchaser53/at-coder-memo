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
    x:usize,
    y:usize,
    z:usize
  }

  let x = x - z;
  println!("{}", x / (y+z));
}
