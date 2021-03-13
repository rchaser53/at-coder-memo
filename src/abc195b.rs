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
const MAX: usize = 1000;

fn main() {
  input!{
    a: usize,
    b: usize,
    w: usize
  }
  let w = w * 1000;

  let mut max = 0;
  let mut min = 1_000_000_000;
  for i in 1..=1_000_000 {
    if a*i <= w && w <= b*i {
      max = std::cmp::max(max, i);
      min = std::cmp::min(min, i);
    }
  }

  if max == 0 {
    println!("UNSATISFIABLE"); 
  } else {
    println!("{} {}", min, max);  
  }
}