/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:[isize;n]
  }

  let mut memo = vec![0isize;n+1];
  for i in 1..=n {
    memo[i] = memo[i-1] + a[(i+1)/2-1];
  }

  let mut dp = vec![vec![-9999999;n+1];n+1];
  dp[1][0] = 0;
  for i in 1..n {
    for j in 0..=n {
      if dp[i][j] < 0 {
        continue
      }
      dp[i+1][j+1] = dp[i+1][j+1].max(dp[i][j]);
      dp[i+1][0] = dp[i+1][0].max(dp[i][j] + memo[j]);
    }
  }

  let mut result = 0;
  for i in 0..n {
    result = result.max(dp[n][i] + memo[i]);
  }

  println!("{}", result);
}