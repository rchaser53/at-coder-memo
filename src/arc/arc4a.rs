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

fn main() {
  input!{
    n: usize,
    vals: [(f64, f64);n]
  }
  
  let mut result = 0f64;
  for i in 0..n {
    let (x1, y1) = vals[i];
    for ii in i+1..n {
      let (x2, y2) = vals[ii];
      let nv = ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
      if result < nv {
        result = nv;
      }
    }
  }
  println!("{}", result);
}