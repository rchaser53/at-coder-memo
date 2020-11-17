#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;
fn main() {
  input!{
    h: usize,
    w: usize,
    rows: [Chars;h]
  }
  
  let mut dp = vec![vec![0;w];h];
  let mut dpx = vec![vec![0;w];h];
  let mut dpy = vec![vec![0;w];h];
  let mut dpz = vec![vec![0;w];h];
  dp[0][0] = 1;

  for i in 0..h {
    for ii in 0..w {
      if rows[i][ii] == '#' || (i==0 && ii==0) { continue }
      if 0 < i {
        dpy[i][ii] = (dp[i-1][ii] + dpy[i-1][ii]) % MOD;
      }
      if 0 < ii {
        dpx[i][ii] = (dp[i][ii-1] + dpx[i][ii-1]) % MOD;
      }
      if 0 < i && 0 < ii {
        dpz[i][ii] = (dp[i-1][ii-1] + dpz[i-1][ii-1]) % MOD;
      }
      
      dp[i][ii] = (dpx[i][ii] + dpy[i][ii]) % MOD;
      dp[i][ii] += dpz[i][ii];
      dp[i][ii] %= MOD;
    }
  }
  
  println!("{}", dp[h-1][w-1]);
}