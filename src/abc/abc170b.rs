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
    x:usize,
    y:usize
  }

  for i in 0..=x {
    if y == i * 4 + (x-i) * 2 {
      println!("Yes");
      return
    }
  }
  println!("No");
}