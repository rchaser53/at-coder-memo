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
    k: usize,
    vals: [usize;n]
  }
  
  if k == 1 {
    println!("{}", n);
    return
  }
  
  let mut count = 0;
  let mut que = VecDeque::new();
  
  for v in vals {
    if que.is_empty() {
      que.push_back(v);
    } else {
      if *que.back().unwrap() < v {
        que.push_back(v);
        if que.len() == k {
          que.pop_front();
          count += 1;
        }
      } else {
        que = VecDeque::new();
        que.push_back(v);
      }
    }
  }
  
  println!("{}", count);
}