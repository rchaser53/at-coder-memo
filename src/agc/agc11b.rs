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
    mut vals:[usize;n]
  }
  vals.sort();


  let mut memo = vec![0;n];
  for i in 0..n-1 {
    memo[i+1] = memo[i] + vals[i];
  }

  for i in (0..n-1).rev() {
    if (vals[i] + memo[i]) * 2 < vals[i+1] {
      println!("{}", n - i - 1);
      return
    }
  }
  println!("{}", n);
}
