#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    k: usize,
    vals: [usize;n]
  }
  
  let mut dp = vec![false;k+vals[n-1]+1];
  
  for i in vals[0]-1..=k {
    for ii in 0..n {
      dp[i] |= vals[ii] <= i
        && !dp[i-vals[ii]];
    }
  }
  if dp[k] {
    println!("First");
  } else {
    println!("Second");
  }
}