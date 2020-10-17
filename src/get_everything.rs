#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input! {
    n: usize,
    m: usize,
  }
  
  let default_value = 10_000_000;
  let limit = 2usize.pow(n as u32);
  let mut dp: Vec<usize> = vec![default_value;limit];
  dp[0] = 0;
  for i in 0..m {
    input! {
      a: usize,
      b: usize,
      vals: [Usize1;b]
    }
    let t = vals.iter().map(|&x| 1 << x).fold(0, |acc, x| acc | x);
    for ii in 0..limit {
      let target_index = ii | t;
      if target_index < limit {
        dp[target_index] = std::cmp::min(dp[target_index], dp[ii] + a);
      }
    }
  }

  if dp[limit-1] == default_value {
    println!("-1");
  } else {
    println!("{}", dp[limit-1]);
  }
}