#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use itertools::Itertools;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};
use std::collections::*;
use superslice::*;

const MOD:usize = 1_000_000_007;
const MAX: usize = 1000;

fn main() {
  input!{
    n: usize,
    mut vals: [usize;n]
  }
  vals.sort();
  let mut dp = vec![vec![0;n+1];4];
  dp[0] = (0..=n).collect_vec();
  for i in 1..4 {
    for ii in 0..n {
      let p = vals.upper_bound(&(vals[ii]/2));
      dp[i][ii+1] = dp[i-1][p] + dp[i][ii];
      dp[i][ii+1] %= MOD;
    }
  }
  
  println!("{}", dp[3][n]%MOD);
}