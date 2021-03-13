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
    vals:[f64;n]
  }

  let mut total = vals.iter().sum::<f64>();
  let average = total / (n as f64);
  let mut target = 0;
  let mut min = 1_000f64;
  for i in 0..n {
    let v = (vals[i] - average).abs();
    if v < min {
      min = v;
      target = i;
    }
  }
  println!("{}", target);
}