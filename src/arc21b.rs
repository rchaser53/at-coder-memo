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
    n:usize,
    vals:[usize;n]
  }
  
  let mut result = vec![0;n];
  for i in 1..n {
    result[i] = vals[i-1] ^ result[i-1];
  }
  
  if result[0] ^ result[n-1] != vals[n-1] {
    println!("-1");
  } else {
    for v in result {
      println!("{}", v);
    }
  }  
}
