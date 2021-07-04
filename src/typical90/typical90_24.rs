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
const MAX:usize = 200_000;

#[fastout]
fn main() {
  input!{
    n:usize,
    k:isize,
    a:[isize;n],
    b:[isize;n],
  }
  
  let mut total = 0;
  for i in 0..n {
    total += (a[i]-b[i]).abs();
  }
  
  if k < total {
    println!("No");
  } else {
    let diff = k - total;
    if diff % 2 == 0 {
      println!("Yes");    
    } else {
      println!("No");
    }
  }
}