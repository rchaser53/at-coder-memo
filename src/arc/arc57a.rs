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
    mut a:usize,
    k:usize
  }
  
  let limit = 2*10usize.pow(12);
  if k == 0 {
    println!("{}", limit - a);
    return
  }
  
  let mut i = 0;
  while a < limit {
    a += 1 + k * a;
    i += 1;
  }
  println!("{}", i);
}