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
    vals: [f64;n],
  }
  
  let mut last = vals[0];
  let mut total = 0f64;
  for i in 1..n {
    let v = vals[i] - last;
    total += vals[i] - last;
    last = vals[i];
  }
  println!("{:.10}", total / ((n - 1) as f64));
}