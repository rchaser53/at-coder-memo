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

const MOD:usize = 998244353;
const MAX:usize = 200_000;

#[fastout]
fn main() {
  input!{
    n:usize,
    k:usize,
  }

  if k == 1 {
    println!("{}", n - 1);
    return
  }
  
  let mut memo = vec![0;n+1];
  for i in 2..=n {
    if memo[i] == 0 {
      let mut j = 2;
      while i*j <= n {
        memo[i*j] += 1;
        j += 1;
      }
    }
  }
  
  let mut count = 0;
  for v in memo {
    if k <= v {
      count += 1;
    }
  }
  println!("{}", count);
}
