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
    n:f64,
    m:f64,
    d:f64
  }
  
  if d == 0f64 {
    println!("{}", (m-1f64) / n);
  } else {
    println!("{}", (m-1f64) * 2f64 * (n-d) / n / n);
  }
}