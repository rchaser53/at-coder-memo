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
    k:usize,
    vals:[usize;n]
  }
  let mut total = 0;
  for i in 0..3 {
    total += vals[i];
  }
  
  if total < k {
    println!("3");
    return
  }
  
  for i in 3..n {
    total -= vals[i-3];
    total += vals[i];
    if total < k {
      println!("{}", i+1);
      return
    }
  }
  println!("-1"); 
}