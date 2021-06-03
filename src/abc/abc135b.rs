/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::{input};
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    vals:[usize;n]
  }
  let mut copied = vals.clone();
  copied.sort();

  let mut count = 0;
  for i in 0..n {
    if vals[i] != copied[i] {
      count += 1;
    }
  }
  
  if count <= 2 {
    println!("YES");
  } else {
    println!("NO");
  }
}