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

fn main() {
  input!{
    n:usize,
    vals:[Usize1;n]
  }  
  
  let mut tree: UnionFind<usize> = UnionFind::new(n);
  for i in 0..n {
    tree.union(i, vals[i]);
  }
    
  let mut set = HashSet::new();
  for i in 0..n {
    set.insert(tree.find(i));
  }

  let mut count = 1;
  for _ in 0..set.len() {
    count *= 2;
    count %= MOD;
  }
  
  println!("{}", count - 1);
}
