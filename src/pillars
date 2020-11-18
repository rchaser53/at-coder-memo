#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    n: usize,
    vals: [isize;n]
  }
  
  let mut dp = vec![0;n];
  dp[1] = (vals[1] - vals[0]).abs();
  for i in 2..n {
    dp[i] = std::cmp::min(
      (vals[i]-vals[i-1]).abs() + dp[i-1],
      (vals[i]-vals[i-2]).abs() + dp[i-2]
    );
  }
  
  println!("{}", dp[n-1]);
}