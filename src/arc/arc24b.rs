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
  input! {
    n: usize,
    vals: [usize;n]
  }
  
  let mut max = 1;
  let mut now = vals[0];
  let mut temp = 1;
  for i in 1..2*n {
    if now == vals[i%n] {
      temp += 1;
    } else {
      now = vals[i%n];
      max = std::cmp::max(max, temp);
      temp = 1;
    }
  }
  max = std::cmp::max(max, temp);
  
  if n <= max {
    println!("-1");
  } else {
    if max <= 2 {
      println!("1");
    } else {
      println!("{}", (max-1) / 2 + 1);
    }
  }
}