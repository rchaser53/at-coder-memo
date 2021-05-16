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

#[fastout]
fn main() {
  input! {
    n: usize,
    vals: [usize;n]
  }
  
  let mut set = HashSet::new();
  let mut left = 0;
  let mut max = 1;
  for i in 0..n {
    let v = vals[i];
    if set.contains(&v) {
      max = std::cmp::max(max, set.len());
      while vals[left] != v {
        set.remove(&vals[left]);
        left += 1;
      }
      left += 1;
    } else {
      set.insert(v);
    }
  }
  max = std::cmp::max(max, set.len());
  println!("{}", max);
}