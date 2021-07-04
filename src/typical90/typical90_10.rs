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
use superslice::*;
use std::cmp::Ordering;
use num_complex::Complex;
 
const MOD:usize = 998244353;
const MAX:usize = 400010;

pub fn main(
) {
input! {
    n:usize,
    vals:[(Usize1,usize);n],
    q:usize,
    queries:[(usize,usize);q]
  }
    
  let mut memo = vec![vec![0;n+1];2];
  for i in 0..n {
    let (ti, v) = vals[i];
    memo[ti][i+1] += v;
    memo[0][i+1] += memo[0][i];
    memo[1][i+1] += memo[1][i];
  }

  for (l, r) in queries {
    println!("{} {}",
      memo[0][r] - memo[0][l-1],
      memo[1][r] - memo[1][l-1],
    );
  }
}