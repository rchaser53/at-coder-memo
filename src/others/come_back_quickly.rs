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
use superslice::Ext;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    m: usize,
    vals: [(Usize1,Usize1,usize);m]
  }
  
  let mut memo = vec![vec![1_000_000_000;n];n];
  for i in 0..m {
    let (l, r, v) = vals[i];
    memo[l][r] = std::cmp::min(memo[l][r], v);
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
  
  for i in 0..n {
    if 1_000_000_000 <= memo[i][i] {
      println!("-1");
    } else {
      println!("{}", memo[i][i]);
    }
  }
}