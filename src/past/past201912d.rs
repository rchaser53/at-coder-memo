#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    vals: [usize;n]
  }
  let mut memo = vec![0;n+1];
  for v in vals {
    memo[v] += 1;
  }
  
  let mut val = 300_000;
  let mut index = 300_000;
  for i in 1..=n {
    if memo[i] == 0 {
      index = i;
    } else if memo[i] == 2 {
      val = i;
    }
  }
  
  if val != 300_000 {
    println!("{} {}", val, index);  
  } else {
    println!("Correct");
  }
}