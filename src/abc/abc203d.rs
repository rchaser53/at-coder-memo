/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn culc(
  rows: &Vec<Vec<usize>>,
  mid:usize,
  n:usize,
  k:usize,
  limit:usize
) -> bool {
  let mut dp = vec![vec![0;n+1];n+1];
    
  for i in 1..=n {
    for j in 1..=n {
      let v = if mid <= rows[i-1][j-1] {
        1
      } else {
        0
      };
      dp[i][j] = dp[i-1][j] + dp[i][j-1] - dp[i-1][j-1] + v;
    }
  }

  for i in 0..n {
    if n < i+k { break }
    for j in 0..n {
      if n < j+k { break }
      let v = dp[k+i][k+j] - dp[k+i][j] - dp[i][k+j] + dp[i][j];
      if v < limit {
        return false
      }
    }
  }
  true
}

pub fn main(
) {
  input! {
    n:usize,
    k:usize,
    rows:[[usize;n];n]
  }

  let mut left = 0usize;
  let mut right = 1_000_000_001;
  let limit = k * k / 2 + 1;
  while left + 1 < right {
    let mid = (left+right)/2;
    if culc(&rows, mid, n, k, limit) {
      left = mid;
    } else {
      right = mid;
    }
  }

  println!("{}", left);
}
