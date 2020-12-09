#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};

const MOD:usize = 10_007;

fn main() {
  input!{
    n: usize,
  }
  
  if n <= 2 {
    println!("0");
    return
  }
  
  let mut dp = vec![0;n+1];
  dp[3] = 1;
  
  for i in 4..=n {
    dp[i] = (dp[i-1] + dp[i-2]) % MOD;
    dp[i] += dp[i-3];
    dp[i] %= MOD;
  }
  
  println!("{}", dp[n]);
}