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

#[fastout]
fn main() {
  input!{
    n: usize,
    m: usize,
    mut vals: [usize;m]
  }
  
  vals.reverse();
  let mut set = HashSet::new();
  let mut should_remove = vec![];
  for i in 0..m {
    if set.contains(&vals[i]) {
      should_remove.push(i);
    }
    set.insert(vals[i]);
  }
  for i in (0..should_remove.len()).rev() {
    vals.remove(should_remove[i]);
  }
  for i in 1..=n {
    if !set.contains(&i) {
      vals.push(i);
    }
  }
  for v in vals {
    println!("{}", v);
  }
}