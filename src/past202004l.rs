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
    k: usize,
    d: usize,
    vals: [usize;n]
  }
  
  let without_first = n - 1;
  let need = d * (k - 1) + 1;
  if n < need {
    println!("-1");
    return
  }
  
  let mut cur = 0;
  let mut pos = 0;
  let mut result = vec![];
  let mut que = BinaryHeap::new();
  for i in 0..k {
    let end = n - d * (k - 1 - i);
    for ii in cur..end {
      que.push(Reverse((vals[ii], ii)));
    }
    
    while let Some(Reverse((val, pos2))) = que.pop() {
      if pos2 < pos {
        continue;
      }
      result.push(val);
      pos = pos2 + d;
      cur = std::cmp::max(pos, end);
      break
    }
  }
  
  for v in result {
    print!("{} ", v);
  }
}