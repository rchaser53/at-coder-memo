/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
    input! {
      n:usize,
      vals:[(usize, usize);n]
    }
    let mut count = 0usize;
    for (l, r) in vals {
      count += r - l + 1;
    }

    println!("{}", count);
}
