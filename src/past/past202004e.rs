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
    vals: [Usize1;n]
  }
  
  for i in 0..n {
    let mut seen = HashSet::new();
    let mut ni = i;
    while !seen.contains(&ni) {
      seen.insert(ni);
      ni = vals[ni];
    }
    println!("{}", seen.len());
  }
}