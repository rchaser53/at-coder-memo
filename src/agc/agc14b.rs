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
    m:usize,
    edges:[(Usize1,Usize1);m]
  }
  let mut memo = vec![0;n];
  for (a, b) in edges {
    memo[a] += 1;
    memo[b] += 1;
  }

  for v in memo {
    if v % 2 == 1 {
      println!("NO");
      return
    }
  }
  println!("YES");
}
