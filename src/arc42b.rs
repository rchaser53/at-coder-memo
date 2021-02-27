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
 
const MOD:usize = 998244353;

#[fastout]
fn main() {
  input!{
    x: f64,
    y: f64,
    n: usize,
    vals: [(f64,f64);n]
  }
  
  let mut min = 1_000_000_000f64;
  for i in 0..n {
    let a = vals[i%n];
    let b = vals[(i+1)%n];
    let ab = (b.0 - a.0, b.1 - a.1);
    let ac = (x - a.0, y - a.1);
    let k = (ab.0 * ac.0 + ab.1 * ac.1) / (ab.0.powi(2) + ab.1.powi(2));
    let ch_len = (k * ab.0 - ac.0).powi(2) + (k * ab.1 - ac.1).powi(2);
    let result = ch_len.sqrt();
    if result < min {
      min = result;
    }
  }
  println!("{}", min);
}