#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    m: usize,
    base: [(Chars, usize);m]
  }
  
  let mut def = vec![(0,0);m];
  for i in 0..m {
    let mut total = 0;
    for ii in 0..n {
      if base[i].0[ii] == 'Y' {
        total |= (1 << ii);
      }
    }
    def[i] = (total, base[i].1);
  }
  
  let limit = 1 << n;
  let inf = 100_000_000_000;
  let mut dp = vec![inf;limit+10];
  dp[0] = 0;
  
  for i in 0..limit {
    for ii in 0..m {
      let (pattern, val) = def[ii];
      let ri = i | pattern;
      dp[ri] = std::cmp::min(dp[ri], dp[i] + val);
    }
  }

  if dp[limit-1] == inf {
    println!("-1");
  } else {
    println!("{}", dp[limit-1]);
  }
}