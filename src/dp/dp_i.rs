/* OUTPUT FILE */
#![allow(unused_imports)]
use petgraph::algo::dijkstra;
use petgraph::graph::{DiGraph, NodeIndex, UnGraph};
use proconio::input;
use proconio::marker::*;
use std::cmp::Reverse;
use std::collections::*;

pub fn main(
) {
    input! {
      n:usize,
      vals:[f64;n]
    }

    let mut dp = vec![vec![0f64;n+1];n+1];
    // 0:失敗 1:成功
    dp[0][0] = 1f64;
    dp[1][0] = 1f64 - vals[0];
    dp[1][1] = vals[0];

    for i in 1..n {
      for j in 0..=i {
        // 成功
        dp[i+1][j+1] += dp[i][j] * vals[i];
        // 失敗
        dp[i+1][j] += dp[i][j] * (1f64 - vals[i]);
      }
    }

    let mut result = 0f64;
    for i in n/2+1..=n {
      result += dp[n][i];
    }
    println!("{}", result);
}
