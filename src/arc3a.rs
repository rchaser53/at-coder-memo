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
    n: f64,
    s: Chars
  }
  
  let mut v = 0f64;
  for c in s {
    match c {
      'A' => v += 4f64,
      'B' => v += 3f64,
      'C' => v += 2f64,
      'D' => v += 1f64,
      _ => {}
    }
  }
  
  println!("{}", v / n);
}