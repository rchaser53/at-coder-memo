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
const MAX: usize = 1000;

fn main() {
  input!{
    n: usize,
    a: f64,
    b: f64,
    vals: [f64;n]
  }
  
  let mut total = 0f64;
  let mut min = 1_000_000_007f64;
  let mut max = 0f64;
  for v in vals {
    if v < min {
      min = v;
    }
    if max < v {
      max = v;
    }
    total += v;
  }
  
  if min == max {
    println!("-1");
    return
  }
  
  let diff = max - min;
  let p = b / diff;
  let average = a * n as f64;
  let diff_total = average - total * p;
  let q = diff_total / n as f64;
  
  println!("{} {}", p, q);
}