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
use std::cmp::*;

const MOD:usize = 1_000_000_007;

#[fastout]
fn main() {
  input!{
    n: usize,
    a: [usize;n],
    b: [usize;n]
  }
  
  let mut aa = 0usize;
  let mut max = 0usize;
  
  for i in 0..n {
    aa = std::cmp::max(aa, a[i]);
    max = std::cmp::max(max, aa * b[i]);
    println!("{}", max);
  } 
}