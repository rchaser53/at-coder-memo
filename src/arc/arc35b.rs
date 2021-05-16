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

fn culc(n: usize) -> usize {
  let mut result = 1;
  for v in 2..=n {
    result *= v;
    result %= MOD;
  }
  result
}

fn main() {
  input!{
    n: usize,
    vals: [usize;n]
  }
  
  let mut bmap = BTreeMap::new();
  for v in vals {
    *bmap.entry(v).or_insert(0) += 1;
  }
  
  let mut total = 0;
  let mut now = 0;
  let mut pattern = 1;
  for (key, num) in bmap {
    for v in 0..num {
      now += key;
      total += now;
    }
    pattern *= culc(num);
    pattern %= MOD;
  }
  
  println!("{}", total);
  println!("{}", pattern);
}