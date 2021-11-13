#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;

#[fastout]
fn main() {
  input!{
    n: usize,
    vals: [(isize, isize, isize);n]
  }

  let limit = 2usize.pow(n as u32);
  let mut dp = vec![vec![1_000_000_000;n];limit];
  dp[0][0] = 0;
  for i in 0..limit {
    for ii in 0..n {
      if i >> ii & 1 == 1 {
        for iii in 0..n {
          if iii != ii {
            let v = (vals[iii].0 - vals[ii].0).abs()
              + (vals[iii].1 - vals[ii].1).abs()
              + std::cmp::max(0, vals[ii].2 - vals[iii].2);
            dp[i][ii] = std::cmp::min(
              dp[i - 2usize.pow(ii as u32)][iii] + v,
              dp[i][ii]
            );
          }
        }
      }
    }
  }
  println!("{}", dp[limit-1][0]);
}