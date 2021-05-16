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
    n: usize,
    m: usize,
    k: usize,
    heights: [usize;n],
    goals: [Usize1;k],
    routes: [(Usize1, Usize1);m]
  }
  
  let mut neighbors = vec![vec![];n];
  for (a, b) in routes {
    if heights[a] > heights[b] {
      neighbors[b].push(a);
    } else {
      neighbors[a].push(b);
    }
  }
  
  let inf = 1_000_000_000;
  let mut dp = vec![inf;n];
  let mut que = VecDeque::new();
  for goal in goals {
    dp[goal] = 0;
    que.push_back(goal);
  }
  
  while !que.is_empty() {
    let i = que.pop_front().unwrap();
    for ii in 0..neighbors[i].len() {
      let ti = neighbors[i][ii];
      let nv  = dp[i] + 1;
      if nv < dp[ti] {
        dp[ti] = nv;
        que.push_back(ti);
      }
    }
  }
  
  for v in dp {
    if v == inf {
      println!("-1");
    } else {
      println!("{}", v);
    }
  }
}