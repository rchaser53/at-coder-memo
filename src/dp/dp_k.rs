/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    k:usize,
    vals:[usize;n]
  }

  let mut memo = vec![false;k+1];
  for i in 0..k+1 {
    for &v in &vals {
      if i + v <= k {
        memo[i+v] |= !memo[i];
      }
    }
  }

  if memo[k] {
    println!("First");
  } else {
    println!("Second");
  }
  
}
