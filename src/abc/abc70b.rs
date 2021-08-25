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
    a:isize,
    b:isize,
    c:isize,
    d:isize
  }

  let left = std::cmp::max(a, c);
  let right = std::cmp::min(b, d);
  if right <= left {
    println!("0")
  } else {
    println!("{}", right - left);
  }
}
