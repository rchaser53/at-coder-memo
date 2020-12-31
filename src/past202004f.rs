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
    mut vals: [(Usize1, usize);n]
  }
  
  vals.sort_by(|a, b| a.0.cmp(&b.0));
  
  let mut total = 0;
  let mut heap = BinaryHeap::new();
  let mut ci = 0;
  for i in 0..n {
    while ci < n && vals[ci].0 == i {
      heap.push(vals[ci].1);
      ci += 1;
    }
    total += heap.pop().unwrap();
    println!("{}", total);
  }
}