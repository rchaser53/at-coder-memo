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
    m: usize,
    k: usize,
    heights: [usize;n],
    goals: [Usize1;k],
    routes: [(Usize1, Usize1);m]
  }
  
  let mut groups = vec![vec![];n];
  for (mut a, mut b) in routes {
    if heights[a] < heights[b] {
      groups[a].push(b);
    } else {
      groups[b].push(a);
    }
  }
  let inf = 100_000_000;
  let mut dp = vec![inf;n];
  let mut que = VecDeque::new();
  for goal in goals {
    dp[goal] = 0;
    que.push_back(goal);
  }
  
  while let Some(v) = que.pop_front() {
    let d = dp[v] + 1;
    for &u in groups[v].iter() {
      if d < dp[u] {
        dp[u] = d;
        que.push_back(u);
      }
    }
  }
  
  for d in dp {
    if d == inf {
      println!("-1");
    } else {
      println!("{}", d);
    }
  } 
}