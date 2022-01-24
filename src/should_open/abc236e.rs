/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    vals:[f64;n]
  }

  let mut left = 0f64;
  let mut right = 10f64.powi(9) + 10f64;
  while 0.000001 < (right - left) {
    let mid = (right + left) / 2f64;
    let mut dp = vec![vec![0f64;2];n+1];
    for i in 0..n {
      let v = vals[i] - mid;
      dp[i+1][0] = if dp[i][1] < dp[i][0] {
        dp[i][0] + v
      } else {
        dp[i][1] + v
      };
      dp[i+1][1] = dp[i][0];
    }

    let temp = if dp[n][0] < dp[n][1] {
      dp[n][1]
    } else {
      dp[n][0]
    };
    
    if 0f64 < temp {
      left = mid;
    } else {
      right = mid;
    }
  }
  println!("{}", left);
  
  let vals = vals.into_iter().map(|v| v as isize).collect::<Vec<isize>>();
  let mut left = -1_000_000_010;
  let mut right = 1_000_000_010;
  while left + 1 < right {
    let mid = (right + left) / 2;
    let mut dp = vec![vec![0;2];n+1];
    for i in 0..n {
      let v = if mid <= vals[i] {
        1
      } else {
        -1
      };
      dp[i+1][0] = std::cmp::max(dp[i][0], dp[i][1]) + v;
      dp[i+1][1] = dp[i][0];
    }

    let temp = if dp[n][0] < dp[n][1] {
      dp[n][1]
    } else {
      dp[n][0]
    };
    
    if 0 < temp {
      left = mid;
    } else {
      right = mid;
    }
  }
  println!("{}", left);

}