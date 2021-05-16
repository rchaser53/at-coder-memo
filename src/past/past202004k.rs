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
use std::cmp::*;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    s: Chars,
    changes: [usize;n],
    deletes: [usize;n]
  }
  
  let default = 1_000_000_000_000_000;
  let mut dp = vec![vec![default;n+1];n+1];
  dp[0][0] = 0;
  for i in 0..n {
    for ii in 0..=n {
      dp[i+1][ii] = std::cmp::min(dp[i+1][ii], dp[i][ii] + deletes[i]);
    }
    for ii in 0..n {
      if s[i] == '(' {
        dp[i+1][ii+1] = std::cmp::min(dp[i+1][ii+1], dp[i][ii]);
      } else {
        dp[i+1][ii+1] = std::cmp::min(dp[i+1][ii+1], dp[i][ii] + changes[i]);
      }
    }
    for ii in 1..=n {
      if s[i] == '(' {
        dp[i+1][ii-1] = std::cmp::min(dp[i+1][ii-1], dp[i][ii] + changes[i]);
      } else {
        dp[i+1][ii-1] = std::cmp::min(dp[i+1][ii-1], dp[i][ii]);
      }
    }
  }
  println!("{}", dp[n][0]);
}