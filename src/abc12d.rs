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
    m: usize,
    vals: [(Usize1, Usize1, usize);m]
  }
  
  let g = UnGraph::<usize, usize, usize>::from_edges(&vals);  

  let mut min = 1_000_000_000;
  for i in 0..n {
    let mut max = 0;
    let res = dijkstra(&g, i.into(), None, |e| *e.weight());
    for ii in 0..n {
      if i == ii { continue }
      if let Some(v) = res.get(&NodeIndex::new(ii)) {
        max = std::cmp::max(max, *v);
      }
    }
    min = std::cmp::min(min, max);
  }
  
  println!("{}", min);
}

fn main() {
  input!{
    n: usize,
    m: usize,
    vals: [(Usize1, Usize1, usize);m]
  }
  
  let mut memo = vec![vec![1_000_000;n];n];
  for i in 0..m {
    let (l, r, v) = vals[i];
    memo[l][r] = v;
    memo[r][l] = v;
  }
  
  for i in 0..n {
    memo[i][i] = 0;
  }
  
  for i in 0..n {
    for j in 0..n {
      for k in 0..n {
        memo[j][k] = std::cmp::min(
          memo[j][k], memo[j][i] + memo[i][k]
        );
      }
    }
  }
  
  let mut min = 1_000_000_000;
  for i in 0..n {
    let mut max = 0;
    for j in 0..n {
      if i == j { continue }
      max = std::cmp::max(max, memo[i][j]);
    }
    min = std::cmp::min(min, max);
  }
  
  println!("{}", min);
}
