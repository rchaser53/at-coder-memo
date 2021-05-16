#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    vals: [usize;n]
  }
  
  for i in 1..n {
    if vals[i-1] < vals[i] {
      println!("up {}", vals[i] - vals[i-1]);
    } else if vals[i-1] > vals[i] {
      println!("down {}", vals[i-1] - vals[i]);    
    } else {
      println!("stay");
    }
  }
}