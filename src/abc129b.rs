#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;
#[fastout]
fn main() {
  input!{
    n:usize,
    vals:[isize;n]
  }
  let mut memo = vec![0;n+1];
  for i in 0..n {
    memo[i+1] = vals[i] + memo[i];
  }
  
  let mut min = 1_000_000_000;
  for i in 1..n {
    min = std::cmp::min(min, (memo[n] - memo[i] - memo[i]).abs());
  }
  println!("{}", min);
}