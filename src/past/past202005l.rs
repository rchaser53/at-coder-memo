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
  }
  
  let mut rows = vec![VecDeque::new();n];
  for i in 0..n {
    input!{
      k: usize,
      row: [usize;k]
    }
    rows[i] = VecDeque::from(row);
  }
  
  input!{
    m: usize,
    customers: [usize;m]
  }

  let mut b_map1 = BTreeMap::new();
  let mut b_map2 = BTreeMap::new();
  let mut cpi = vec![0;n];
  for i in 0..n {
    if let Some(v) = rows[i].get(0) {
      b_map1.insert(Reverse(v), i);
      b_map2.insert(Reverse(v), i);
    }
  }
  for i in 0..n {
    if let Some(v) = rows[i].get(1) {
      b_map2.insert(Reverse(v), i);
    }
  }
  
  for q in customers {
    let (val, ci) = if q == 1 {
      let (Reverse(val), ci) = b_map1.iter().next().unwrap().clone();
      (*val, *ci)
    } else {
      let (Reverse(val), ci) = b_map2.iter().next().unwrap().clone();
      (*val, *ci)
    };

    if let Some(_) = b_map1.remove(&Reverse(val)) {
      if let Some(v) = rows[ci].get(cpi[ci] + 1) {
        b_map1.insert(Reverse(v), ci);      
      }
    }
    if let Some(_) = b_map2.remove(&Reverse(val)) {
      if let Some(v) = rows[ci].get(cpi[ci] + 2) {
        b_map2.insert(Reverse(v), ci);      
      }
    }
    cpi[ci] += 1;
    println!("{}", val);
  }
}
  