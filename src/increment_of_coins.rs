#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    a: usize,
    b: usize,
    c: usize,
  }
  
  let mut dp = vec![vec![vec![0f64;101];101];101];  
  for i in (0..100).rev() {
    for j in (0..100).rev() {
      for k in (0..100).rev() {
        if i + j + k == 0 { continue }
        let mut now = 0f64;
        now += dp[i+1][j][k] * (i as f64);
        now += dp[i][j+1][k] * (j as f64);
        now += dp[i][j][k+1] * (k as f64);
        dp[i][j][k] = now / (i + j + k) as f64 + 1f64;
      }
    }
  }
  println!("{}", dp[a][b][c]);
}
