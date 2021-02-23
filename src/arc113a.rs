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
    k:usize,
  }
  
  let mut count = 0;
  for i in 1..=k {
    for ii in 1..=k {
      let v = i * ii;
      if k < v { break }
      count += k / v; 
    }
  }
  println!("{}", count);
}