/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

pub fn main(
) {
  input! {
    n:usize,
    k:usize,
    m:usize,
    vals:[usize;n-1]
  }

  let total = n * m;
  let now_val = vals.iter().sum::<usize>();
  if total <= now_val {
    println!("0");
  } else {
    let diff = total - now_val;
    if diff <= k {
      println!("{}", diff);
    } else {
      println!("-1");
    }
  }
}
