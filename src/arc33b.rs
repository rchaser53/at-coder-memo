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
  input!{
    na: usize,
    nb: usize,
    a: [usize;na],
    b: [usize;nb]
  }
  
  let mut a_set = HashSet::new();
  let mut b_set = HashSet::new();
  let mut a_or_b = HashSet::new();
  let mut a_and_b = HashSet::new();
  for v in a {
    a_set.insert(v);
    a_or_b.insert(v);
  }
  for v in b {
    b_set.insert(v);
    a_or_b.insert(v);
  }
  for v in a_set {
    if b_set.contains(&v) {
      a_and_b.insert(v);
    }
  }
  println!("{}", (a_and_b.len() as f64) / (a_or_b.len() as f64));
}