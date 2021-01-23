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
    m: usize,
    mut vals: [(usize, Usize1);m]
  }
  vals.sort_by(|a, b| a.1.cmp(&b.1));
  
  let mut result = 0;
  let mut i = 0;
  while i < vals.len() {
    let (_, to) = vals[i];
    i += 1;
    while i < vals.len() {
      let (from, _) = vals[i];
      if to < from { break }
      i += 1;
    }
    result += 1;
  }
  println!("{}", result);
}