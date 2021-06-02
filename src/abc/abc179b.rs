/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::{input};
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    vals:[(usize,usize);n]
  }
  let mut count = 0;
  for (l,r) in vals {
    if l == r {
      count += 1;
    } else {
      count = 0;
    }

    if count == 3 {
      println!("Yes");
      return
    }
  }
  println!("No");
}