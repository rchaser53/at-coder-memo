/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::{input};
use proconio::marker::*;
use itertools::Itertools;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    q:usize,
    vals:[isize;n],
    queries:[(Usize1,Usize1,isize);q]
  }
  let mut result = 0;
  let mut memo = vec![0;n+1];
  for i in 0..n-1 {
    memo[i] = vals[i+1] - vals[i];
    result += memo[i].abs();
  }

  for (l, r, v) in queries {
    let mut bv = 0;
    let mut av = 0;
    if 0 < l {
      bv += memo[l-1].abs();
      memo[l-1] += v;
      av += memo[l-1].abs();
    }
    if r < n-1 {
      bv += memo[r].abs();
      memo[r] -= v;
      av += memo[r].abs();
    }
    result = result - bv + av;

    println!("{}", result);
  }
}
