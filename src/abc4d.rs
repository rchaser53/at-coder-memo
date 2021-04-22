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


#[fastout]
fn main() {
  input!{
    r:usize,
    g:usize,
    b:usize
  }
  let inf = 1 << 60;
  let mut dp = vec![vec![inf;r+g+b+10];2000];
  dp[0][0] = 0;
  let mut result = inf;
  for i in 0..1900 {
    for j in 0..r+g+b+1 {
      let v = if j < r {
        (900 - i as isize).abs() as usize
      } else if j < r + g {
        (1000 - i as isize).abs() as usize
      } else {
        (1100 - i as isize).abs() as usize
      };
      dp[i+1][j+1] = std::cmp::min(dp[i][j] + v, dp[i+1][j+1]);
      dp[i+1][j] = std::cmp::min(dp[i+1][j], dp[i][j]);
    }
    result = std::cmp::min(dp[i][r+g+b], result);
  }
  println!("{}", result);
}