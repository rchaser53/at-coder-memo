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
    a:usize,
    b:usize
  }

  let diff = b - a;
  let mut tot = 0usize;
  for i in 1..=diff {
    tot += i;
  }
  println!("{}", tot-b);
}
