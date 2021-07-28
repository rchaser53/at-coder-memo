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
    mut vals: [usize;n]
  }

  let mut result = 0;
  let mut last = 0;
  for v in vals {
    if v == 0 {
      last = 0;
    } else {
      last += v;
      result += last / 2;
      last %= 2;
    }
  }
  println!("{}", result);
}