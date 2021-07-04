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
use superslice::*;
use std::cmp::Ordering;
use num_complex::Complex;
 
const MOD:usize = 998244353;
const MAX:usize = 400010;

fn main() {
  input!{
    n:usize,
    vals:[String;n]
  }
  
  let mut set = HashSet::new();
  for i in 0..n {
    if !set.contains(&vals[i]) {
      set.insert(&vals[i]);
      println!("{}", i+1);
    }
  }
}