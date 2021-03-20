#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use itertools::Itertools;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};
use superslice::*;
use std::collections::*;
use std::cmp::*;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    mut n:usize,
    m:usize,
    a:usize,
    b:usize,
    vals:[usize;m]
  }
  
  for i in 0..m {
    if n <= a {
      n += b;
    }
    if n < vals[i] {
      println!("{}", i+1);
      return
    }
    n -= vals[i];
  }
  println!("complete");
}