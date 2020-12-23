#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    m: usize,
    vals: [usize;m]
  }
  
  let mut memo = vec![0;n];  
  for i in 0..m {
    let v = match memo.binary_search(&vals[i]) {
        Ok(v) => v,
        Err(v) => v
    };
    
    if v == 0 {
      println!("-1");
    } else {
      memo[v-1] = vals[i];
      println!("{}", n - v + 1);
    }
  }
} 
