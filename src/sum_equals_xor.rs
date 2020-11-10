#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;
#[fastout]
fn main() {
  input!{
    l: Chars
  }

  let mut dp = vec![vec![0;2];l.len()+1];
  dp[0][0] = 1;  
  for i in 1..=l.len() {
    if l[i-1] == '1' {
      // 1-0, 0-1
      dp[i][0] = dp[i-1][0] * 2 % MOD;
      dp[i][1] = (dp[i-1][1] * 3 % MOD + dp[i-1][0]) % MOD;
    } else {
      dp[i][0] = dp[i-1][0];
      dp[i][1] = dp[i-1][1] * 3 % MOD;
    }
  }
  
  println!("{}", (dp[l.len()][0] + dp[l.len()][1]) % MOD);
}