#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;

#[fastout]
fn main() {
  input!{
    n: usize,
    s: Chars
  }
  
  let mut dp: Vec<Vec<isize>> = vec![vec![0;n];n];
  
  for i in 0..n {
    if s[i] == s[n-1] {
      dp[n-1][i] = 1;
      dp[i][n-1] = 1;
    }
  }
  
  let mut max = 0;
  for i in (0..n-1).rev() {
    for j in (0..n-1).rev() {
      if s[i] == s[j] {
        dp[i][j] = dp[i+1][j+1] + 1;
        let v = std::cmp::min(
          ((i as isize) - (j as isize)).abs(),
          dp[i][j]
        );
        max = std::cmp::max(max, v);
      }
    }
  }
  println!("{}", max);
}