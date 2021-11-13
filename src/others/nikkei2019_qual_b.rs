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
    n: usize,
    a: Chars,
    b: Chars,
    c: Chars
  }
  
  let mut count = 0;
  for i in 0..n {
    let mut set = HashSet::new();
    set.insert(a[i]);
    set.insert(b[i]);
    set.insert(c[i]);
    
    count += set.len() - 1;
  }
  println!("{}", count);
  
}