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
use superslice::Ext;

const MOD:usize = 1_000_000_007;

#[fastout]
fn main() {
  input!{
    n: usize,
    m: usize,
    vals: [usize;n]
  }
  
  let mut memo = vec![0;n];
  for i in 0..m {
    memo[vals[i]] += 1;
  }
  let mut min = n;
  for i in 0..n {
    if memo[i] == 0 {
      min = i;
      break
    }
  }
  for i in 0..n-m {
    memo[vals[m+i]] += 1;
    memo[vals[i]] -= 1;
    if memo[vals[i]] == 0 {
      min = std::cmp::min(min, vals[i]);
    }
  }
  println!("{}", min);
}
