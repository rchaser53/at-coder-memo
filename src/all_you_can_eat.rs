#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input! {
    n: usize,
    t: usize,
    mut vals: [(usize, usize);n]
  }
  vals.sort_by(|a,b| a.0.cmp(&b.0));
  
  let mut dp: Vec<Vec<usize>> = vec![vec![0;t+3000];n+1];
  for i in 0..n {
    let (time, val) = vals[i];
    for ii in 0..t {      
      dp[i+1][ii+time] = std::cmp::max(dp[i][ii] + val, dp[i+1][ii+time]);
      dp[i+1][ii] = std::cmp::max(dp[i+1][ii], dp[i][ii]);
    }
  }
  let mut max = 0;
  for row in dp {
    max = std::cmp::max(row.into_iter().max().unwrap(), max);
  }

  println!("{}", max);
}