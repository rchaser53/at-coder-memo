#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    l: usize,
    mut hardles: [usize;n],
    times: [usize;3]
  }
  
  let mut dp = vec![1_000_000_000_000;l+10];
  dp[0] = 0;
  let mut blocks = HashSet::new();
  for i in 0..n {
    blocks.insert(hardles[i]);
  }
  
  for i in 0..=l {
    let mut v1 = times[0];
    let mut v2 = times[1] + times[0];
    let mut v3 = times[1] * 3 + times[0];
    if blocks.contains(&i) {
      v1 += times[2];
      v2 += times[2];
      v3 += times[2];
    }
    dp[i+1] = std::cmp::min(dp[i+1], dp[i] + v1);
    dp[i+2] = std::cmp::min(dp[i+2], dp[i] + v2);
    dp[i+4] = std::cmp::min(dp[i+4], dp[i] + v3);    
  }
  
  let start = l.saturating_sub(3);
  for i in start..l {
    let diff = l - i - 1;
    let mut v = 0;
    if blocks.contains(&i) {
      v += times[2];
    }
    dp[l] = std::cmp::min(
      dp[l],
      dp[i] + times[0] / 2 + times[1] * diff + times[1] / 2 + v
    );
  }

  println!("{}", dp[l]);
}