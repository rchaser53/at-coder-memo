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

const MOD:usize = 1_000_000_007;
const MAX: usize = 1000;

#[fastout]
fn main() {
  input!{
    n:usize,
    mut a:[isize;n],
    mut b:[isize;n]
  }
  a.sort();
  b.sort();
  
  let mut total = 0;
  for i in 0..n {
    total += (a[i]-b[i]).abs();
  }
  println!("{}", total);
}