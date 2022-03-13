/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    m:usize,
    edges:[(Usize1,Usize1,usize);m]
  }

  let inf = 1_000_000_000_000_000usize;
  let mut memo = vec![vec![inf;n];n];

  for (l, r, c) in edges {
    memo[l][r] = c;
  }
  for i in 0..n {
    memo[i][i] = 0;
  }

  // ワーシャルフロイド Warshall Floyd
  let mut result = 0usize;
  for k in 0..n {
    for i in 0..n {
      for j in 0..n {
        memo[i][j] = std::cmp::min(memo[i][j], memo[i][k] + memo[k][j]);
        if memo[i][j] != inf {
          result += memo[i][j];
        }
      }
    }
  }
  println!("{}", result);
}
